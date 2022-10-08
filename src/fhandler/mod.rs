#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumString;
use regex::Regex;
use crate::tsp::Euc2d;

#[derive(Debug, PartialEq, EnumString)]
enum EdgeWeightType {
    #[strum(serialize = "EUC_2D")]
    Euc2D,
    #[strum(serialize = "EXPLICIT")]
    Explicit
}

const DIMENSION_PATTERN: &str = r"DIMENSION\s*:\s*(\d+)";
const EDGE_WEIGHT_PATTERN: &str = r"EDGE_WEIGHT_TYPE\s*:\s*(\w+)";

#[inline]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_tsp_file(path: String) -> (usize, Vec<Euc2d>, Vec<Vec<i64>>) {

    let dim_regex = Regex::new(DIMENSION_PATTERN).unwrap();
    let edge_type_regex = Regex::new(EDGE_WEIGHT_PATTERN).unwrap();
    let mut load_weights = false;

    let mut weight_matrix: Vec<Vec<i64>> = Vec::new();
    let mut cities: Vec<Euc2d> = Vec::new();
    let mut dimension: usize = 0;

    cities.push(Euc2d { x_coord: 0.0_f64, y_coord: 0.0_f64 }); // invalid 0-index city

    if let Ok(file_iter) = read_lines(path) {

        for line in file_iter {

            if let Ok(instruction) = line {

                if load_weights {

                    let data: Vec<f64> = instruction.trim().split_whitespace()
                        .flat_map(str::parse::<f64>).collect();
                    cities.push(Euc2d { x_coord: data[1], y_coord: data[2] });
                    

                    if data[0] as i32 == dimension as i32 {
                        load_weights = false;
                    }
                }
                else if instruction.starts_with("DIMENSION") {

                    dimension = dim_regex.captures(instruction.as_str()).unwrap()
                        .get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
                    
                    // Initing weights matrix
                    for _ in 0..=dimension {
                        weight_matrix.push(vec![0; dimension+1]);
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

    (dimension, cities, weight_matrix)
}