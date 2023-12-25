use crate::node::{Hash256, Node};
#[derive(Debug)]
pub struct MerkleTree {
    root_node: Box<Node>,
}

impl MerkleTree {
    pub fn new(depth: usize, initial_leaf: Hash256) -> Self {
        let root_node = Node::new(depth, initial_leaf);
        let root_node = Box::new(root_node);
        MerkleTree { root_node }
    }

    pub fn root(&self) -> &Hash256 {
        self.root_node.hash()
    }
}
