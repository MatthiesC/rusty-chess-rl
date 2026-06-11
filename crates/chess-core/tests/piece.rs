use chess_core::{Color, Piece, PieceKind};

#[test]
fn colors_are_opposites() {
    assert_eq!(Color::White.opposite(), Color::Black);
    assert_eq!(Color::Black.opposite(), Color::White);
}

#[test]
fn piece_preserves_color_and_kind() {
    let piece = Piece::new(Color::White, PieceKind::Knight);

    assert_eq!(piece.color(), Color::White);
    assert_eq!(piece.kind(), PieceKind::Knight);
}
