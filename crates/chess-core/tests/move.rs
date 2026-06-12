//! Integration tests for rule-agnostic move descriptions and application.

use chess_core::{Board, Color, Move, MoveError, MoveOutcome, Piece, PieceKind, Square};

fn square(file: u8, rank: u8) -> Square {
    Square::new(file, rank).expect("test square must be valid")
}

#[test]
fn move_preserves_source_and_destination() {
    let from: Square = square(4, 1);
    let to: Square = square(4, 3);
    let chess_move: Move = Move::new(from, to);

    assert_eq!(chess_move.from(), from);
    assert_eq!(chess_move.to(), to);
}

#[test]
fn applies_basic_move_without_rule_validation() {
    let mut board: Board = Board::starting_position();
    let from: Square = square(0, 0);
    let to: Square = square(4, 4);
    let rook: Piece = Piece::new(Color::White, PieceKind::Rook);

    let outcome: MoveOutcome = board
        .apply_move(Move::new(from, to))
        .expect("source contains a piece");

    assert_eq!(outcome.moved(), rook);
    assert_eq!(outcome.captured(), None);
    assert_eq!(board.piece_at(from), None);
    assert_eq!(board.piece_at(to), Some(rook));
}

#[test]
fn applying_move_replaces_and_reports_destination_piece() {
    let mut board: Board = Board::starting_position();
    let from: Square = square(0, 0);
    let to: Square = square(0, 7);
    let white_rook: Piece = Piece::new(Color::White, PieceKind::Rook);
    let black_rook: Piece = Piece::new(Color::Black, PieceKind::Rook);

    let outcome: MoveOutcome = board
        .apply_move(Move::new(from, to))
        .expect("source contains a piece");

    assert_eq!(outcome.moved(), white_rook);
    assert_eq!(outcome.captured(), Some(black_rook));
    assert_eq!(board.piece_at(from), None);
    assert_eq!(board.piece_at(to), Some(white_rook));
}

#[test]
fn empty_source_returns_error_without_changing_board() {
    let mut board: Board = Board::starting_position();
    let before: Board = board.clone();
    let from: Square = square(4, 3);
    let to: Square = square(4, 4);

    let result: Result<MoveOutcome, MoveError> = board.apply_move(Move::new(from, to));

    assert_eq!(result, Err(MoveError::EmptySource(from)));
    assert_eq!(board, before);
}

#[test]
fn same_square_move_succeeds_without_changing_board() {
    let mut board: Board = Board::starting_position();
    let before: Board = board.clone();
    let square: Square = square(1, 0);
    let knight: Piece = Piece::new(Color::White, PieceKind::Knight);

    let outcome: MoveOutcome = board
        .apply_move(Move::new(square, square))
        .expect("source contains a piece");

    assert_eq!(outcome.moved(), knight);
    assert_eq!(outcome.captured(), None);
    assert_eq!(board, before);
}
