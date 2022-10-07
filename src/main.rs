mod fhandler;
mod pow;

use std::env;
use fhandler::*;
use pow::*;

const TSPLIB_INST_FOLDER: &str = "./tsplib95/ALL_tsp/";

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut fpath = String::from(TSPLIB_INST_FOLDER);
    fpath.push_str(&args[1]);

    let mut distances: Vec<Vec<i64>> = Vec::new();
    let mut cities: Vec<Euc2d> = Vec::new();
    

    let dimension = parse_tsp_file(fpath, &mut distances, &mut cities);
    generate_circuit(dimension);
    let genesis = build_genesis_block(dimension);
    index(genesis.blockhash, 10, dimension);
    
}
