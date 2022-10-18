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

    pub fn init(cities: Vec<Euc2d>, distances: Vec<Vec<i64>>, dimension: usize) -> Self {

        let mut me = Self {
            dimension, cities, distances
        };

        me.load_distances();

        me
    }

    pub fn load_distances(&mut self) {

        for row in 1..=self.dimension {

            for col in 1..=self.dimension {
    
                // Euclidian Distance
                self.distances[row][col] = f64::sqrt(
                    f64::powi(self.cities[col].x_coord - self.cities[row].x_coord, 2) +
                    f64::powi(self.cities[col].y_coord - self.cities[row].y_coord, 2)).round() as i64;
            }
        }
    }

    fn random_append(&self, circuit: &mut Vec<u32>, from: usize, exto: usize, dim: usize) {

        let mut is_selected = vec![false; exto];
        let mut rng = rand::thread_rng();
 
        for _ in 0..dim {
    
            let mut city;
            
            loop {
                city = rng.gen_range(from..exto);
                if !is_selected[city] {
                    break;
                }
            }
    
            circuit.push(city as u32);
            is_selected[city] = true;
        }
    }

    pub fn generate_circuit(&self) -> Vec<u32> {

        let mut circuit: Vec<u32> = Vec::new();

        circuit.push(0);
        self.random_append(&mut circuit, 1, self.dimension + 1, self.dimension);
        circuit[0] = circuit[self.dimension];
    
        circuit
    }

    pub fn evaluate_circuit(&self, circuit: &Vec<u32>) -> i64 {

        let mut total: i64 = 0;

        for i in 0..circuit.len() - 1 {
            total += self.distances[circuit[i] as usize][circuit[i + 1] as usize];
        }

        total
    }

    pub fn generate_neighbor(&self, original: &Vec<u32>, ksubspace: &Vec<u32>) -> Vec<u32> {

        let mut neighbor = original.clone();
        let kdim = ksubspace.len();

        let mut neighidx = Vec::new();
        self.random_append(&mut neighidx, 0, kdim, kdim);

        let mut neighsubspace = vec![0; kdim];
        for nidx in 0..kdim {
            neighsubspace[neighidx[nidx] as usize] = ksubspace[nidx];
        }
        println!("NSubspace: {:?}", neighsubspace);

        for nidx in 0..kdim {
            neighbor[neighsubspace[nidx] as usize] = original[ksubspace[nidx] as usize];
        }

        neighbor[original.len() - 1] = neighbor[0];

        neighbor
    }

}