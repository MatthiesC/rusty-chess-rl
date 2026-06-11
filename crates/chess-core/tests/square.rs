use chess_core::Square;

#[test]
fn creates_valid_squares() {
    let a1 = Square::new(0, 0).expect("a1 is valid");
    let h8 = Square::new(7, 7).expect("h8 is valid");

    assert_eq!(a1.index(), 0);
    assert_eq!(h8.index(), 63);
    assert_eq!((h8.file(), h8.rank()), (7, 7));
}

#[test]
fn rejects_out_of_bounds_coordinates_and_indices() {
    assert_eq!(Square::new(8, 0), None);
    assert_eq!(Square::new(0, 8), None);
    assert_eq!(Square::from_index(64), None);
}

#[test]
fn index_round_trip_preserves_every_square() {
    for index in 0..Square::COUNT {
        let square = Square::from_index(index).expect("index is on the board");

        assert_eq!(Square::new(square.file(), square.rank()), Some(square));
    }
}
