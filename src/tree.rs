use crate::node::{Node, NodeHash, Proof};

#[derive(Debug)]
pub struct MerkleTree<H: NodeHash> {
    depth: u32,
    root_node: Box<Node<H>>,
}

impl<H: NodeHash> MerkleTree<H> {
    pub fn new(depth: u32, initial_leaf: H) -> Self {
        let root_node = Node::new(depth, initial_leaf);
        let root_node = Box::new(root_node);
        MerkleTree { depth, root_node }
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
}
