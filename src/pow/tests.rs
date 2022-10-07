use super::*;

#[test]
fn is_circuit_correctly_initiated() {

    let dim: usize = 5;
    let circuit = generate_circuit(dim);

    for i in 0..dim-1 {

        assert!(circuit[i] <= dim as u32);

        for j in i + 1..dim {
            assert_ne!(circuit[i], circuit[j]);
        }
    }

    assert_eq!(circuit[0], circuit[dim]);
    assert_eq!(circuit.len(), dim + 1);
} 