use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumString;
use regex::Regex;

const TSPLIB_INST_FOLDER: &str = "./tsplib95/ALL_tsp/";

#[derive(Debug, PartialEq, EnumString)]
enum EdgeWeightType {
    #[strum(serialize = "EUC_2D")]
    Euc2D,
    #[strum(serialize = "EXPLICIT")]
    Explicit
}

struct Euc2d {
    x_coord: i32,
    y_coord: i32
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let mut path = String::from(TSPLIB_INST_FOLDER);
    path.push_str(&args[1]);


    let dim_regex = Regex::new(r"DIMENSION\s*:\s*(\d+)").unwrap();
    let edge_type_regex = Regex::new(r"EDGE_WEIGHT_TYPE\s*:\s*(\w+)").unwrap();
    let mut _weight_matrix: Vec<Vec<u32>> = Vec::new();
    let mut cities_coord: Vec<Euc2d> = Vec::new();
    let mut _tsp_circuit: Vec<u32> = Vec::new();
    let mut load_weights = false;
    let mut dimension: usize = 0;

    if let Ok(file_iter) = read_lines(path) {

        for line in file_iter {

            if let Ok(instruction) = line {

                if load_weights {

                    let data: Vec<i32> = instruction.trim().split_whitespace()
                        .flat_map(str::parse::<i32>).collect();
                    cities_coord.push(Euc2d { x_coord: data[1], y_coord: data[2] });
                    

                    if data[0] == dimension as i32 {
                        load_weights = false;
                    }
                }
                else if instruction.starts_with("DIMENSION") {
                    dimension = dim_regex.captures(instruction.as_str()).unwrap()
                        .get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap());

                    _tsp_circuit = Vec::with_capacity(dimension + 1);
                    _weight_matrix = Vec::with_capacity(dimension);
                    for _ in 0..dimension {
                        _weight_matrix.push(Vec::with_capacity(dimension));
                    }
                }
                else if instruction.starts_with("EDGE_WEIGHT_TYPE") {
                    let edge_type = edge_type_regex.captures(instruction.as_str()).unwrap()
                        .get(1).map_or("", |m| m.as_str());
                    let weight_type = EdgeWeightType::from_str(edge_type).unwrap();
                    assert_eq!(EdgeWeightType::Euc2D, weight_type);
                }
                else if instruction.starts_with("NODE_COORD_SECTION") {
                    load_weights = true;
                }
            }
        }
    }
}
