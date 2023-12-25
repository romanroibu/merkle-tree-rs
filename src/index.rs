#[derive(PartialEq, Eq, Debug)]
pub struct Index(pub u32);

#[derive(PartialEq, Eq, Debug)]
pub struct Location {
    pub depth: u32,
    pub offset: u32,
}
