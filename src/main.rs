mod fhandler;
mod pow;
mod tsp;

use std::env;
use fhandler::*;
use tsp::*;

const TSPLIB_INST_FOLDER: &str = "./tsplib95/ALL_tsp/";

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut fpath = String::from(TSPLIB_INST_FOLDER);
    fpath.push_str(&args[1]);

    let (dimension, cities, distances) = parse_tsp_file(fpath);
    let tspinst = TSP::init(cities, distances, dimension);

    let circuit = tspinst.generate_circuit();
    println!("Loop: {:?}", circuit);
    println!("Circuit value: {}", tspinst.evaluate_circuit(&circuit));
    
    let genesis = pow::build_genesis_block(circuit);
    let ksubspace = pow::get_index(&genesis.blockhash, 5, 14);
    println!("Ksubspace: {:?}", ksubspace);

    let neighbor = tspinst.generate_neighbor(&genesis.header.circuit, &ksubspace);
    println!("Neighbor: {:?}", neighbor);
    println!("Neighbor value: {}", tspinst.evaluate_circuit(&neighbor));
}
