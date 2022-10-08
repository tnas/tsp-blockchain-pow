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

#[test]
fn index_subspace() {

    let hash = String::from("4fc0fc4638ae261dbabf49eba978869a7dfa5dc56f3a58d5f91f04307a85001f");
    let kdim: u32 = 10;
    let dim: usize = 280;
    let ksubspace = get_index(hash, kdim, dim);
    let expected:Vec<u32> = vec![175, 86, 117, 7, 149, 65, 191, 12, 0, 261];

    for i in 0..expected.len() {
        assert_eq!(expected[i], ksubspace[i]);
    }
}