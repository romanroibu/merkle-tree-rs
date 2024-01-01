use merkle_tree::MerkleTree;

mod hex_value;
use hex_value::HexValue;

#[test]
fn test_tree_set_with_depth_0() {
    let initial_leaf = hex!("0000000000000000000000000000000000000000000000000000000000000000");
    let updated_leaf = hex!("1111111111111111111111111111111111111111111111111111111111111111");

    let mut tree = MerkleTree::new(0, initial_leaf.clone()).unwrap();
    assert_eq!(tree.root(), &initial_leaf);

    tree.set(0, updated_leaf.clone());
    assert_eq!(tree.root(), &updated_leaf);
}

#[test]
fn test_tree_set_with_depth_1() {
    let initial_leaf = hex!("0000000000000000000000000000000000000000000000000000000000000000");
    let updated_leaf = hex!("1111111111111111111111111111111111111111111111111111111111111111");

    let mut tree = MerkleTree::new(1, initial_leaf.clone()).unwrap();
    assert_eq!(
        tree.root(),
        &hex!("070fa1ab6fcc557ed14d42941f1967693048551eb9042a8d0a057afbd75e81e0")
    );

    tree.set(0, updated_leaf.clone());
    assert_eq!(
        tree.root(),
        &hex!("aa679f6812decf546048d3d610fe9b3567d8c4b6d2e4013360180617c10acccc")
    );

    tree.set(1, updated_leaf.clone());
    assert_eq!(
        tree.root(),
        &hex!("b3ec7cf4503854f1f691ffb3c0bde5e22af4705161edb20ede25a62e3209a716")
    );

    tree.set(0, initial_leaf.clone());
    tree.set(1, initial_leaf.clone());
    assert_eq!(
        tree.root(),
        &hex!("070fa1ab6fcc557ed14d42941f1967693048551eb9042a8d0a057afbd75e81e0")
    );
}

#[test]
fn test_tree_set_example_1_depth_1() {
    let initial_leaf = hex!("0000000000000000000000000000000000000000000000000000000000000000");
    let mut tree = MerkleTree::new(1, initial_leaf).unwrap();
    tree.set(
        0,
        hex!("0000000000000000000000000000000000000000000000000000000000000000"),
    );
    tree.set(
        1,
        hex!("1111111111111111111111111111111111111111111111111111111111111111"),
    );
    assert_eq!(
        tree.root(),
        &hex!("35e794f1b42c224a8e390ce37e141a8d74aa53e151c1d1b9a03f88c65adb9e10")
    );

    let mut tree = MerkleTree::new(
        1,
        hex!("0000000000000000000000000000000000000000000000000000000000000000"),
    )
    .unwrap();
    tree.set(
        0,
        hex!("2222222222222222222222222222222222222222222222222222222222222222"),
    );
    tree.set(
        1,
        hex!("3333333333333333333333333333333333333333333333333333333333333333"),
    );
    assert_eq!(
        tree.root(),
        &hex!("777d6e92478a47e81651fcd03f9f3aec04893589a865a171cb17127fdee83f64")
    );
}

#[test]
fn test_tree_set_with_depth_4() {
    let initial_leaf = hex!("0000000000000000000000000000000000000000000000000000000000000000");
    let mut tree = MerkleTree::new(4, initial_leaf).unwrap();

    tree.set(
        0,
        hex!("0000000000000000000000000000000000000000000000000000000000000000"),
    );
    tree.set(
        1,
        hex!("1111111111111111111111111111111111111111111111111111111111111111"),
    );
    tree.set(
        2,
        hex!("2222222222222222222222222222222222222222222222222222222222222222"),
    );
    tree.set(
        3,
        hex!("3333333333333333333333333333333333333333333333333333333333333333"),
    );
    tree.set(
        4,
        hex!("4444444444444444444444444444444444444444444444444444444444444444"),
    );
    tree.set(
        5,
        hex!("5555555555555555555555555555555555555555555555555555555555555555"),
    );
    tree.set(
        6,
        hex!("6666666666666666666666666666666666666666666666666666666666666666"),
    );
    tree.set(
        7,
        hex!("7777777777777777777777777777777777777777777777777777777777777777"),
    );
    tree.set(
        8,
        hex!("8888888888888888888888888888888888888888888888888888888888888888"),
    );
    tree.set(
        9,
        hex!("9999999999999999999999999999999999999999999999999999999999999999"),
    );
    tree.set(
        10,
        hex!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
    );
    tree.set(
        11,
        hex!("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"),
    );
    tree.set(
        12,
        hex!("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC"),
    );
    tree.set(
        13,
        hex!("DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD"),
    );
    tree.set(
        14,
        hex!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE"),
    );
    tree.set(
        15,
        hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"),
    );

    assert_eq!(
        tree.root(),
        &hex!("57054e43fa56333fd51343b09460d48b9204999c376624f52480c5593b91eff4")
    );
}
