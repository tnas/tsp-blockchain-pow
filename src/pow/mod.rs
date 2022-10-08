#[cfg(test)]
mod tests;

use hex;
use sha2::{Sha256, Digest};
use num::{bigint::{BigUint, ToBigUint}, Integer, ToPrimitive, Zero};

pub struct Block {
    pub prev_blockhash: String,
    pub blockhash: String,
    pub header: BlockHeader
}

pub struct BlockHeader {
    height: u32,
    difficulty: f64,
    circuit: Vec<u32>
}


fn hash_string(word: String) -> String {

    let hash = Sha256::new()
        .chain_update(word)
        .finalize();

    format!("{:x}", hash)
}

fn hash_block(block: &Block) -> String {
    
    let mut hasher = Sha256::new()
        .chain_update(block.prev_blockhash.as_bytes())
        .chain_update(u32::to_ne_bytes(block.header.height))
        .chain_update(f64::to_ne_bytes(block.header.difficulty));
    block.header.circuit.iter().for_each(|e| hasher.update(u32::to_be_bytes(*e)));
    

    let hash = hasher.finalize();

    format!("{:x}", hash)
}


pub fn build_genesis_block(circuit: Vec<u32>) -> Block {

    let genesis_header = BlockHeader {
        height: 0,
        difficulty: 0.0,
        circuit: circuit
    };

    let mut genesis = Block {
        header: genesis_header,
        blockhash: String::new(),
        prev_blockhash: String::new()
    };

    genesis.blockhash = hash_block(&genesis);

    genesis
}

pub fn get_index(hash: String, kdim: u32, dim: usize) -> Vec<u32> {

    let mut ksubspace = Vec::new();

    let xbytes = hex::decode(hash.clone());
    let mut dhash = BigUint::from_bytes_be(&xbytes.ok().unwrap());
 
    let ubig_dim = ToBigUint::to_biguint(&0xF).unwrap();
    
    while ksubspace.len() < kdim as usize {

        let kelement = 
            dhash.mod_floor(&ToBigUint::to_biguint(&dim).unwrap()).to_u32().unwrap();
            
        if !ksubspace.contains(&kelement) {
            ksubspace.push(kelement);
        }
    
        dhash = dhash.div_floor(&ubig_dim);

        if dhash == BigUint::zero() {
            let hhash = hash_string(hash.clone());
            dhash = BigUint::from_bytes_be(&hex::decode(hhash).ok().unwrap());
        }
    }

    ksubspace
}