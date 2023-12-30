use merkle_tree::MerkleTree;

mod hex_value;
use hex_value::HexValue;

#[test]
fn test_tree_new_with_depth_0() {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("abababababababababababababababababababababababababababababababab");
    assert_eq!(MerkleTree::new(0, initial_leaf).root(), &expected_root);
}

#[test]
fn test_tree_new_with_depth_1() {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("699fc94ff1ec83f1abf531030e324003e7758298281645245f7c698425a5e0e7");
    assert_eq!(MerkleTree::new(1, initial_leaf).root(), &expected_root);
}

#[test]
fn test_tree_new_with_depth_19() {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("d4490f4d374ca8a44685fe9471c5b8dbe58cdffd13d30d9aba15dd29efb92930");
    assert_eq!(MerkleTree::new(19, initial_leaf).root(), &expected_root);
}

#[test]
fn test_tree_new_with_depth_20() {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("2e83853566789d4b59a7a6f48e4ee98da67addcb7d194cf75b3bcf5833714d1f");
    assert_eq!(MerkleTree::new(20, initial_leaf).root(), &expected_root);
}
