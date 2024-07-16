# [Merkle Tree](https://github.com/romanroibu/merkle-tree-rs/)

Basic implementation of a Merkle Tree in Rust.

## Example

```rust
let initial_leaf = hex!("0000000000000000000000000000000000000000000000000000000000000000");
let leaf_3 = hex!("3333333333333333333333333333333333333333333333333333333333333333");

let mut tree = MerkleTree::new(4, initial_leaf).unwrap();
tree.set(3, leaf_3.clone()).unwrap();

let proof = tree.proof(3).unwrap();
assert_eq!(&MerkleTree::verify(&proof, leaf_3), tree.root());
```
