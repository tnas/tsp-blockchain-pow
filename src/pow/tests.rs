use super::*;

#[test]
fn is_circuit_correctly_initiated() {

    let dim: usize = 5;
    let mut circuit: Vec<u32> = Vec::new();

    generate_circuit(&mut circuit, dim);

    for i in 0..dim-1 {

        assert!(circuit[i] <= dim as u32);

        for j in i + 1..dim {
            assert_ne!(circuit[i], circuit[j]);
        }
    }

    assert_eq!(circuit[0], circuit[dim]);
    assert_eq!(circuit.len(), dim + 1);
} 