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
