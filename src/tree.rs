use crate::node::{Node, NodeHash};

#[derive(Debug)]
pub struct MerkleTree<H: NodeHash> {
    root_node: Box<Node<H>>,
}

impl<H: NodeHash> MerkleTree<H> {
    pub fn new(depth: u32, initial_leaf: H) -> Self {
        let root_node = Node::new(depth, initial_leaf);
        let root_node = Box::new(root_node);
        MerkleTree { root_node }
    }

    pub fn root(&self) -> &H {
        self.root_node.hash()
    }
}
