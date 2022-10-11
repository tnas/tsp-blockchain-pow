use super::*;

#[test]
fn get_hash_string() {
    let word = String::from("I am Satoshi Nakamoto");
    let expected = String::from("5d7c7ba21cbbcd75d14800b100252d5b428e5b1213d27c385bc141ca6b47989e");
    assert_eq!(expected, hash_string(word));
}

#[test]
fn index_subspace() {

    let hash = String::from("4fc0fc4638ae261dbabf49eba978869a7dfa5dc56f3a58d5f91f04307a85001f");
    let kdim: u32 = 10;
    let dim: usize = 280;
    let ksubspace = get_index(&hash, kdim, dim);
    let expected:Vec<u32> = vec![175, 86, 117, 7, 149, 65, 191, 12, 0, 261];

    for i in 0..expected.len() {
        assert_eq!(expected[i], ksubspace[i]);
    }
}