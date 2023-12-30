use merkle_tree::MerkleTree;
use merkle_tree::ProofNode::{Left, Right};

mod hex_value;
use hex_value::HexValue;

#[test]
fn test_tree_set_with_depth_4() {
    let initial_leaf = hex!("0000000000000000000000000000000000000000000000000000000000000000");
    let mut tree = MerkleTree::new(4, initial_leaf.into());

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
        tree.proof(3),
        vec![
            Right {
                sibling: hex!("2222222222222222222222222222222222222222222222222222222222222222")
            },
            Right {
                sibling: hex!("35e794f1b42c224a8e390ce37e141a8d74aa53e151c1d1b9a03f88c65adb9e10")
            },
            Left {
                sibling: hex!("26fca7737f48fa702664c8b468e34c858e62f51762386bd0bddaa7050e0dd7c0")
            },
            Left {
                sibling: hex!("e7e11a86a0c1d8d8624b1629cb58e39bb4d0364cb8cb33c4029662ab30336858")
            },
        ]
    );
}
