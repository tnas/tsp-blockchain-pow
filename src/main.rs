mod fhandler;
mod pow;
mod tsp;

use std::env;
use fhandler::*;

const TSPLIB_INST_FOLDER: &str = "./tsplib95/ALL_tsp/";

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut fpath = String::from(TSPLIB_INST_FOLDER);
    fpath.push_str(&args[1]);

    let (dimension, cities, distances) = parse_tsp_file(fpath);
    let mut tspinst = tsp::init_tsp(cities, distances, dimension);
    tspinst.calculate_distances();

    let genesis = pow::build_genesis_block(tsp::generate_circuit(dimension));
    pow::get_index(genesis.blockhash, 10, dimension);
}
