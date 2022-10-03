mod fhandler;

use std::env;
use fhandler::*;

const TSPLIB_INST_FOLDER: &str = "./tsplib95/ALL_tsp/";

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut path = String::from(TSPLIB_INST_FOLDER);
    path.push_str(&args[1]);

    let mut weight_matrix: Vec<Vec<i64>> = Vec::new();
    let mut cities: Vec<Euc2d> = Vec::new();
    parse_tsp_file(path, &mut weight_matrix, &mut cities);

}
