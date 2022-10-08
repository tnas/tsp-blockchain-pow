#[cfg(test)]
mod tests;

use rand::Rng;

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