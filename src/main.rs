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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let mut path = String::from(TSPLIB_INST_FOLDER);
    path.push_str(&args[1]);

    println!("TSP Instance: {}", path);

    let dim_regex = Regex::new(r"DIMENSION: (\d+)").unwrap();
    let edge_type_regex = Regex::new(r"EDGE_WEIGHT_TYPE: (\w+)").unwrap();
    let mut weight_matrix: Vec<Vec<u16>> = Vec::new();

    if let Ok(file_iter) = read_lines(path) {

        for line in file_iter {

            if let Ok(instruction) = line {

                if instruction.starts_with("DIMENSION") {
                    let dimension = dim_regex.captures(instruction.as_str()).unwrap()
                        .get(1).map_or(0, |m| m.as_str().parse::<u8>().unwrap());
                    println!("--The number of cities: {}", dimension);
                }
                else if instruction.starts_with("EDGE_WEIGHT_TYPE") {
                    let edge_type = edge_type_regex.captures(instruction.as_str()).unwrap()
                        .get(1).map_or("", |m| m.as_str());
                    let weight_type = EdgeWeightType::from_str(edge_type).unwrap();
                    println!("--The edge weight type: {}", edge_type);
                    assert_eq!(EdgeWeightType::Explicit, weight_type);
                }
                else if instruction.starts_with("EDGE_WEIGHT_SECTION") {
                    
                    // for r in 1..29 {
                        
                    // }

                    let row: Vec<u16> = Vec::new();

                    weight_matrix.push(row);       

                    println!("{}", instruction);

                }
                else if instruction.starts_with("NODE_COORD_SECTION") {

                }
            }
        }
    }
}
