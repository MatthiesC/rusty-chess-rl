use chess_core::{Board, Color, FenError, Piece, PieceKind, Position, Square};

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w";

fn square(file: u8, rank: u8) -> Square {
    Square::new(file, rank).expect("test square must be valid")
}

#[test]
fn parses_starting_position() {
    let position: Position = STARTING_FEN.parse().expect("starting FEN must be valid");

    assert_eq!(position.board(), &Board::starting_position());
    assert_eq!(position.side_to_move(), Color::White);
}

#[test]
fn parses_piece_placement_and_black_to_move() {
    let fen: &str = "8/8/3k4/8/8/4Q3/8/8 b";
    let position: Position = fen.parse().expect("FEN must be valid");

    assert_eq!(
        position.board().piece_at(square(3, 5)),
        Some(Piece::new(Color::Black, PieceKind::King))
    );
    assert_eq!(
        position.board().piece_at(square(4, 2)),
        Some(Piece::new(Color::White, PieceKind::Queen))
    );
    assert_eq!(position.side_to_move(), Color::Black);
}

#[test]
fn serializes_starting_position() {
    let position: Position = Position::starting_position();

    assert_eq!(position.to_string(), STARTING_FEN);
}

#[test]
fn round_trip_preserves_supported_position_state() {
    let fen: &str = "r3k2r/ppp2ppp/2n5/3qp3/8/2N2N2/PPP2PPP/R2Q1RK1 b";
    let position: Position = fen.parse().expect("FEN must be valid");
    let serialized: String = position.to_string();
    let reparsed: Position = serialized.parse().expect("serialized FEN must be valid");

    assert_eq!(serialized, fen);
    assert_eq!(reparsed, position);
}

#[test]
fn rejects_wrong_field_count() {
    let result: Result<Position, FenError> = "8/8/8/8/8/8/8/8 w - - 0 1".parse();

    assert_eq!(result, Err(FenError::InvalidFieldCount { found: 6 }));
}

#[test]
fn rejects_wrong_rank_count() {
    let result: Result<Position, FenError> = "8/8/8/8/8/8/8 w".parse();

    assert_eq!(result, Err(FenError::InvalidRankCount { found: 7 }));
}

#[test]
fn rejects_rank_that_is_too_short() {
    let result: Result<Position, FenError> = "7/8/8/8/8/8/8/8 w".parse();

    assert_eq!(
        result,
        Err(FenError::InvalidRankWidth { rank: 8, width: 7 })
    );
}

#[test]
fn rejects_invalid_empty_count() {
    let result: Result<Position, FenError> = "9/8/8/8/8/8/8/8 w".parse();

    assert_eq!(result, Err(FenError::InvalidEmptyCount('9')));
}

#[test]
fn rejects_rank_that_is_too_long() {
    let result: Result<Position, FenError> = "8P/8/8/8/8/8/8/8 w".parse();

    assert_eq!(
        result,
        Err(FenError::InvalidRankWidth { rank: 8, width: 9 })
    );
}

#[test]
fn rejects_invalid_piece_symbol() {
    let result: Result<Position, FenError> = "7x/8/8/8/8/8/8/8 w".parse();

    assert_eq!(result, Err(FenError::InvalidPiece('x')));
}

#[test]
fn rejects_adjacent_empty_counts() {
    let result: Result<Position, FenError> = "44/8/8/8/8/8/8/8 w".parse();

    assert_eq!(result, Err(FenError::AdjacentEmptyCounts { rank: 8 }));
}

#[test]
fn rejects_invalid_active_color() {
    let result: Result<Position, FenError> = "8/8/8/8/8/8/8/8 x".parse();

    assert_eq!(result, Err(FenError::InvalidActiveColor("x".to_owned())));
}
