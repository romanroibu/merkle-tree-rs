use sha3::{Digest, Sha3_256};

pub trait NodeHash: AsRef<[u8]> + From<[u8; 32]> + Into<[u8; 32]> + Clone {}

#[derive(Clone, Debug)]
pub(crate) enum Node<H: NodeHash> {
    Leaf {
        hash: H,
    },
    Branch {
        hash: H,
        left: Box<Node<H>>,
        right: Box<Node<H>>,
    },
}

#[derive(Debug)]
pub(crate) enum NodeError {
    InvalidTree,
}

impl<H: NodeHash> Node<H> {
    pub(crate) fn hash(&self) -> &H {
        match *self {
            Node::Leaf { ref hash, .. } => hash,
            Node::Branch { ref hash, .. } => hash,
        }
    }

    pub(crate) fn new(depth: u32, initial_leaf: H) -> Self {
        match depth {
            0 => Node::Leaf { hash: initial_leaf },
            n => {
                let child = Node::new(n - 1, initial_leaf);
                let child_hash = child.hash();

                let hash = Node::sha3(child_hash, child_hash);
                let left = Box::new(child.clone());
                let right = Box::new(child);
                Node::Branch { hash, left, right }
            }
        }
    }

    pub(crate) fn set_leaf(
        &mut self,
        depth: u32,
        leaf_index: u32,
        leaf_value: H,
    ) -> Result<(), NodeError> {
        match (self, depth) {
            (Node::Leaf { ref mut hash }, 0) => {
                *hash = leaf_value;
                Ok(())
            }
            (
                Node::Branch {
                    ref mut hash,
                    ref mut left,
                    ref mut right,
                },
                1..,
            ) => {
                let brach_left = leaf_index & (0b1 << (depth - 1)) == 0;

                if brach_left {
                    left.set_leaf(depth - 1, leaf_index, leaf_value)?;
                } else {
                    right.set_leaf(depth - 1, leaf_index, leaf_value)?;
                };
                *hash = Node::sha3(left.hash(), right.hash());
                Ok(())
            }
            _ => Err(NodeError::InvalidTree),
        }
    }

    fn sha3(x: &H, y: &H) -> H {
        let mut hasher = Sha3_256::new();
        hasher.update(x);
        hasher.update(y);
        let hash: [u8; 32] = hasher.finalize().into();
        H::from(hash)
    }
}
