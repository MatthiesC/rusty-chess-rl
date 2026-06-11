use chess_core::{Board, Color, Piece, PieceKind, Position, Square};

fn square(file: u8, rank: u8) -> Square {
    Square::new(file, rank).expect("test square must be valid")
}

#[test]
fn creates_position_from_board_and_side_to_move() {
    let board: Board = Board::empty();
    let position: Position = Position::new(board.clone(), Color::Black);

    assert_eq!(position.board(), &board);
    assert_eq!(position.side_to_move(), Color::Black);
}

#[test]
fn starting_position_has_white_to_move() {
    let position: Position = Position::starting_position();

    assert_eq!(position.board(), &Board::starting_position());
    assert_eq!(position.side_to_move(), Color::White);
}

#[test]
fn allows_board_and_side_to_move_updates() {
    let mut position: Position = Position::new(Board::empty(), Color::White);
    let e4: Square = square(4, 3);
    let pawn: Piece = Piece::new(Color::White, PieceKind::Pawn);

    position.board_mut().set_piece(e4, Some(pawn));
    position.set_side_to_move(Color::Black);

    assert_eq!(position.board().piece_at(e4), Some(pawn));
    assert_eq!(position.side_to_move(), Color::Black);
}
