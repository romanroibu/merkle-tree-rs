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

    fn sha3(x: &H, y: &H) -> H {
        let mut hasher = Sha3_256::new();
        hasher.update(x);
        hasher.update(y);
        let hash: [u8; 32] = hasher.finalize().into();
        H::from(hash)
    }
}
