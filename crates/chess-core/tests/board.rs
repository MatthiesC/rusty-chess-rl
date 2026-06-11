use chess_core::{Board, Color, Piece, PieceKind, Square};

fn square(file: u8, rank: u8) -> Square {
    Square::new(file, rank).expect("test square must be valid")
}

#[test]
fn empty_board_contains_no_pieces() {
    let board = Board::empty();

    assert!(board.iter().all(|(_, piece)| piece.is_none()));
}

#[test]
fn starting_position_has_expected_pieces() {
    let board = Board::starting_position();

    assert_eq!(
        board.piece_at(square(0, 0)),
        Some(Piece::new(Color::White, PieceKind::Rook))
    );
    assert_eq!(
        board.piece_at(square(4, 0)),
        Some(Piece::new(Color::White, PieceKind::King))
    );
    assert_eq!(
        board.piece_at(square(3, 7)),
        Some(Piece::new(Color::Black, PieceKind::Queen))
    );
    assert_eq!(
        board.piece_at(square(7, 7)),
        Some(Piece::new(Color::Black, PieceKind::Rook))
    );
    assert_eq!(board.piece_at(square(4, 3)), None);
}

#[test]
fn starting_position_contains_sixteen_pieces_per_color() {
    let board = Board::starting_position();

    let white_count = board
        .iter()
        .filter(|(_, piece)| piece.is_some_and(|piece| piece.color() == Color::White))
        .count();
    let black_count = board
        .iter()
        .filter(|(_, piece)| piece.is_some_and(|piece| piece.color() == Color::Black))
        .count();

    assert_eq!(white_count, 16);
    assert_eq!(black_count, 16);
}

#[test]
fn set_piece_replaces_and_clears_square() {
    let mut board = Board::empty();
    let e4 = square(4, 3);
    let pawn = Piece::new(Color::White, PieceKind::Pawn);

    assert_eq!(board.set_piece(e4, Some(pawn)), None);
    assert_eq!(board.piece_at(e4), Some(pawn));
    assert_eq!(board.set_piece(e4, None), Some(pawn));
    assert_eq!(board.piece_at(e4), None);
}

#[test]
fn iteration_visits_every_square_in_index_order() {
    let board = Board::empty();
    let indices = board.iter().map(|(square, _)| square.index()).collect::<Vec<_>>();

    assert_eq!(indices, (0..Square::COUNT).collect::<Vec<_>>());
}

#[test]
fn renders_empty_board_as_ascii() {
    let board = Board::empty();

    assert_eq!(
        board.to_string(),
        "\
8 . . . . . . . .
7 . . . . . . . .
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 . . . . . . . .
1 . . . . . . . .
  a b c d e f g h"
    );
}

#[test]
fn renders_starting_position_as_ascii() {
    let board = Board::starting_position();

    assert_eq!(
        board.to_string(),
        "\
8 r n b q k b n r
7 p p p p p p p p
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 P P P P P P P P
1 R N B Q K B N R
  a b c d e f g h"
    );
}
