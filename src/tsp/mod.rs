#[cfg(test)]
mod tests;

use std::vec;

use rand::Rng;
pub struct Euc2d {
    pub x_coord: f64,
    pub y_coord: f64
}

pub struct TSP {
    pub dimension: usize,
    pub cities: Vec<Euc2d>,
    pub distances: Vec<Vec<i64>>
}

impl TSP {

    pub fn calculate_distances(&mut self) {

        for row in 1..=self.dimension {

            for col in 1..=self.dimension {
    
                // Euclidian Distance
                self.distances[row][col] = f64::sqrt(
                    f64::powi(self.cities[col].x_coord - self.cities[row].x_coord, 2) +
                    f64::powi(self.cities[col].y_coord - self.cities[row].y_coord, 2)).round() as i64;
            }
        }
    }
}

pub fn init_tsp(cityvec: Vec<Euc2d>, distvec: Vec<Vec<i64>>, dim: usize) -> TSP {

    let tsp = TSP {
        dimension: dim,
        cities: cityvec,
        distances: distvec
    };

    tsp
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