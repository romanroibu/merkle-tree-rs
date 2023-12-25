use sha3::{Digest, Sha3_256};

pub type Hash256 = [u8; 32];

#[derive(Clone, Debug)]
pub(crate) enum Node {
    Leaf {
        hash: Hash256,
    },
    Branch {
        hash: Hash256,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    pub(crate) fn hash(&self) -> &Hash256 {
        match *self {
            Node::Leaf { ref hash, .. } => hash,
            Node::Branch { ref hash, .. } => hash,
        }
    }

    pub(crate) fn new(depth: usize, initial_leaf: Hash256) -> Self {
        match depth {
            0 => Node::Leaf { hash: initial_leaf },
            n => {
                let child = Node::new(n - 1, initial_leaf);
                let child_hash = child.hash();

                let mut hasher = Sha3_256::new();
                hasher.update(child_hash);
                hasher.update(child_hash);
                let hash = hasher.finalize().into();

                let left = Box::new(child.clone());
                let right = Box::new(child);
                Node::Branch { hash, left, right }
            }
        }
    }
}
