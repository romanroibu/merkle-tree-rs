#[derive(PartialEq, Eq, Debug)]
pub struct Index(pub u32);

#[derive(PartialEq, Eq, Debug)]
pub struct Location {
    pub depth: u32,
    pub offset: u32,
}

impl From<Location> for Index {
    fn from(location: Location) -> Self {
        let index = 2_u32.pow(location.depth) - 1 + location.offset;
        Index(index)
    }
}
