use super::*;
use crate::fhandler::init_weight_matrix;

#[test]
fn integer_euclidian_dist() {

    let dim: usize = 3;
    let mut weight_matrix: Vec<Vec<i64>> = Vec::new();
    init_weight_matrix(&mut weight_matrix, dim);

    let mut cities: Vec<Euc2d> = Vec::new();
    cities.push(Euc2d { x_coord: 0.0, y_coord: 0.0 });
    cities.push(Euc2d { x_coord: 64.0, y_coord: 96.0 });
    cities.push(Euc2d { x_coord: 80.0, y_coord: 39.0 });
    cities.push(Euc2d { x_coord: 69.0, y_coord: 23.0 });

    let mut expected: Vec<Vec<i64>> = Vec::new();
    init_weight_matrix(&mut expected, dim);
    expected[1][2] = 59;
    expected[1][3] = 73;
    expected[2][3] = 19;

    let inst = TSP::init(cities, weight_matrix, dim);
    
    assert_eq!(expected[1][2], inst.distances[1][2]);
    assert_eq!(expected[1][3], inst.distances[1][3]);
    assert_eq!(expected[2][3], inst.distances[2][3]);
}

#[test]
fn is_circuit_correctly_initiated() {

    let dim: usize = 5;
    let mut tsp = TSP::new();
    tsp.dimension = dim;
    let circuit = tsp.generate_circuit();

    for i in 0..dim-1 {

        assert!(circuit[i] <= dim as u32);

        for j in i + 1..dim {
            assert_ne!(circuit[i], circuit[j]);
        }
    }

    assert_eq!(circuit[0], circuit[dim]);
    assert_eq!(circuit.len(), dim + 1);
}