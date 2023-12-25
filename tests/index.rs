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

#[test]
fn test_location_from_index() {
    assert_eq!(
        Location::from(Index(0)),
        Location {
            depth: 0,
            offset: 0
        }
    );
    assert_eq!(
        Location::from(Index(1)),
        Location {
            depth: 1,
            offset: 0
        }
    );
    assert_eq!(
        Location::from(Index(2)),
        Location {
            depth: 1,
            offset: 1
        }
    );
    assert_eq!(
        Location::from(Index(3)),
        Location {
            depth: 2,
            offset: 0
        }
    );
    assert_eq!(
        Location::from(Index(4)),
        Location {
            depth: 2,
            offset: 1
        }
    );
    assert_eq!(
        Location::from(Index(5)),
        Location {
            depth: 2,
            offset: 2
        }
    );
    assert_eq!(
        Location::from(Index(6)),
        Location {
            depth: 2,
            offset: 3
        }
    );
    assert_eq!(
        Location::from(Index(7)),
        Location {
            depth: 3,
            offset: 0
        }
    );
    assert_eq!(
        Location::from(Index(8)),
        Location {
            depth: 3,
            offset: 1
        }
    );
    assert_eq!(
        Location::from(Index(9)),
        Location {
            depth: 3,
            offset: 2
        }
    );
    assert_eq!(
        Location::from(Index(10)),
        Location {
            depth: 3,
            offset: 3
        }
    );
    assert_eq!(
        Location::from(Index(11)),
        Location {
            depth: 3,
            offset: 4
        }
    );
    assert_eq!(
        Location::from(Index(12)),
        Location {
            depth: 3,
            offset: 5
        }
    );
    assert_eq!(
        Location::from(Index(13)),
        Location {
            depth: 3,
            offset: 6
        }
    );
    assert_eq!(
        Location::from(Index(14)),
        Location {
            depth: 3,
            offset: 7
        }
    );
}

#[test]
fn test_parent_index_from_index() {
    assert_eq!(Index(0).parent(), Option::None);
    assert_eq!(Index(1).parent(), Option::Some(Index(0)));
    assert_eq!(Index(2).parent(), Option::Some(Index(0)));
    assert_eq!(Index(3).parent(), Option::Some(Index(1)));
    assert_eq!(Index(4).parent(), Option::Some(Index(1)));
    assert_eq!(Index(5).parent(), Option::Some(Index(2)));
    assert_eq!(Index(6).parent(), Option::Some(Index(2)));
    assert_eq!(Index(7).parent(), Option::Some(Index(3)));
    assert_eq!(Index(8).parent(), Option::Some(Index(3)));
    assert_eq!(Index(9).parent(), Option::Some(Index(4)));
    assert_eq!(Index(10).parent(), Option::Some(Index(4)));
    assert_eq!(Index(11).parent(), Option::Some(Index(5)));
    assert_eq!(Index(12).parent(), Option::Some(Index(5)));
    assert_eq!(Index(13).parent(), Option::Some(Index(6)));
    assert_eq!(Index(14).parent(), Option::Some(Index(6)));
}

#[test]
fn test_left_child_index_from_index() {
    assert_eq!(Index(0).left_child(), Index(1));
    assert_eq!(Index(1).left_child(), Index(3));
    assert_eq!(Index(2).left_child(), Index(5));
    assert_eq!(Index(3).left_child(), Index(7));
    assert_eq!(Index(4).left_child(), Index(9));
    assert_eq!(Index(5).left_child(), Index(11));
    assert_eq!(Index(6).left_child(), Index(13));
}
