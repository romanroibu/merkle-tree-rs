use thiserror::Error;

use crate::node::{Node, NodeHash, Proof};

pub const MAX_TREE_DEPTH: u32 = 29;

#[derive(Debug)]
pub struct MerkleTree<H: NodeHash> {
    depth: u32,
    root_node: Box<Node<H>>,
}

#[derive(Debug, Error, PartialEq)]
pub enum MerkleTreeInitError {
    #[error("{0} exceeds maximal tree depth")]
    TooDeep(u32),
}

impl<H: NodeHash> MerkleTree<H> {
    pub fn new(depth: u32, initial_leaf: H) -> Result<Self, MerkleTreeInitError> {
        if depth > MAX_TREE_DEPTH {
            return Err(MerkleTreeInitError::TooDeep(depth));
        }
        let root_node = Node::new(depth, initial_leaf);
        let root_node = Box::new(root_node);
        Ok(MerkleTree { depth, root_node })
    }

    pub fn root(&self) -> &H {
        self.root_node.hash()
    }

    pub fn num_leaves(&self) -> u64 {
        2_u64.pow(self.depth)
    }

    pub fn set(&mut self, leaf_index: u32, leaf_value: H) {
        let root = self.root_node.as_mut();
        root.set_leaf(self.depth, leaf_index, leaf_value).unwrap();
    }

    pub fn proof(&self, leaf_index: u32) -> Proof<H> {
        let mut proof = Vec::with_capacity(self.depth as usize);
        self.root_node
            .generate_proof(self.depth, leaf_index, &mut proof)
            .unwrap();
        proof
    }

    pub fn verify(proof: &Proof<H>, leaf_value: H) -> H {
        Node::verify(proof, leaf_value)
    }
}
