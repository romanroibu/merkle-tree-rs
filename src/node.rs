pub type Hash256 = [u8; 32];

#[derive(Debug)]
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
