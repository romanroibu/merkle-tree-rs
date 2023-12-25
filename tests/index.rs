use merkle_tree::{Index, Location};

#[test]
fn test_index_from_location() {
    assert_eq!(
        Index::from(Location {
            depth: 0,
            offset: 0
        }),
        Index(0)
    );
    assert_eq!(
        Index::from(Location {
            depth: 1,
            offset: 0
        }),
        Index(1)
    );
    assert_eq!(
        Index::from(Location {
            depth: 1,
            offset: 1
        }),
        Index(2)
    );
    assert_eq!(
        Index::from(Location {
            depth: 2,
            offset: 0
        }),
        Index(3)
    );
    assert_eq!(
        Index::from(Location {
            depth: 2,
            offset: 1
        }),
        Index(4)
    );
    assert_eq!(
        Index::from(Location {
            depth: 2,
            offset: 2
        }),
        Index(5)
    );
    assert_eq!(
        Index::from(Location {
            depth: 2,
            offset: 3
        }),
        Index(6)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 0
        }),
        Index(7)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 1
        }),
        Index(8)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 2
        }),
        Index(9)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 3
        }),
        Index(10)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 4
        }),
        Index(11)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 5
        }),
        Index(12)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 6
        }),
        Index(13)
    );
    assert_eq!(
        Index::from(Location {
            depth: 3,
            offset: 7
        }),
        Index(14)
    );
}
