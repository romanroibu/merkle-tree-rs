use merkle_tree::{MerkleTree, MerkleTreeInitError, MAX_TREE_DEPTH};

mod hex_value;
use hex_value::HexValue;

#[test]
fn test_tree_new_with_depth_0() -> Result<(), MerkleTreeInitError> {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("abababababababababababababababababababababababababababababababab");
    let tree = MerkleTree::new(0, initial_leaf)?;
    assert_eq!(tree.root(), &expected_root);
    Ok(())
}

#[test]
fn test_tree_new_with_depth_1() -> Result<(), MerkleTreeInitError> {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("699fc94ff1ec83f1abf531030e324003e7758298281645245f7c698425a5e0e7");
    let tree = MerkleTree::new(1, initial_leaf)?;
    assert_eq!(tree.root(), &expected_root);
    Ok(())
}

#[test]
fn test_tree_new_with_depth_19() -> Result<(), MerkleTreeInitError> {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("d4490f4d374ca8a44685fe9471c5b8dbe58cdffd13d30d9aba15dd29efb92930");
    let tree = MerkleTree::new(19, initial_leaf)?;
    assert_eq!(tree.root(), &expected_root);
    Ok(())
}

#[test]
fn test_tree_new_with_depth_20() -> Result<(), MerkleTreeInitError> {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("2e83853566789d4b59a7a6f48e4ee98da67addcb7d194cf75b3bcf5833714d1f");
    let tree = MerkleTree::new(20, initial_leaf)?;
    assert_eq!(tree.root(), &expected_root);
    Ok(())
}

#[test]
#[ignore]
fn test_tree_new_with_max_depth() -> Result<(), MerkleTreeInitError> {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let expected_root = hex!("f3352b6f19fa606a73c4071bc930068b73430bee1143337216a461b7b1d1b4c1");
    let tree = MerkleTree::new(MAX_TREE_DEPTH, initial_leaf)?;
    assert_eq!(tree.root(), &expected_root);
    Ok(())
}

#[test]
fn test_tree_new_past_max_depth() -> Result<(), MerkleTreeInitError> {
    let initial_leaf = hex!("abababababababababababababababababababababababababababababababab");
    let maybe_tree = MerkleTree::new(MAX_TREE_DEPTH + 1, initial_leaf);

    assert!(maybe_tree.is_err());
    assert_eq!(
        maybe_tree.unwrap_err(),
        MerkleTreeInitError::TooDeep(MAX_TREE_DEPTH + 1)
    );
    Ok(())
}
