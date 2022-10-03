use super::*;

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

    calculate_weights(&mut weight_matrix, &cities, dim);

    assert_eq!(expected[1][2], weight_matrix[1][2]);
    assert_eq!(expected[1][3], weight_matrix[1][3]);
    assert_eq!(expected[2][3], weight_matrix[2][3]);
}