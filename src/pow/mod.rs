#[cfg(test)]
mod tests;

use std::vec;
use rand::Rng;
use sha2::{Sha256, Digest};

pub struct Block {
    prev_blockhash: String,
    blockhash: String,
    header: BlockHeader
}

pub struct BlockHeader {
    height: u32,
    difficulty: f64,
    circuit: Vec<u32>
}

fn hash_block(block: &Block) -> String {
    
    let mut hasher = Sha256::new()
        .chain_update(block.prev_blockhash.as_bytes())
        .chain_update(u32::to_ne_bytes(block.header.height))
        .chain_update(f64::to_ne_bytes(block.header.difficulty));
    block.header.circuit.iter().for_each(|e| hasher.update(u32::to_be_bytes(*e)));
    
    format!("{:x}", hasher.finalize())
}


pub fn build_genesis_block(dim: usize) -> Block {

    let genesis_header = BlockHeader {
        height: 0,
        difficulty: 0.0,
        circuit: generate_circuit(dim)
    };

    let mut genesis = Block {
        header: genesis_header,
        blockhash: String::new(),
        prev_blockhash: String::new()
    };

    genesis.blockhash = hash_block(&genesis);

    genesis
}

pub fn generate_circuit(dim: usize) -> Vec<u32> {

    let mut circuit: Vec<u32> = Vec::new();
    let mut is_selected = vec![false; dim + 1];

    let mut rng = rand::thread_rng();
    let first = rng.gen_range(1..dim + 1);
    circuit.push(first as u32);
    is_selected[first] = true;
    
    for _ in 0..dim {

        let mut city;
        
        loop {
            city = rng.gen_range(1..dim + 1);
            if !is_selected[city] {
                break;
            }
        }

        circuit.push(city as u32);
        is_selected[city] = true;

        if circuit.len() == dim {
            break;
        }
    }

    circuit.push(first as u32);

    circuit
}