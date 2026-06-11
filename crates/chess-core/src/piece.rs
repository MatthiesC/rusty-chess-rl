/// The side to which a chess piece belongs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Color {
    /// The white side.
    White,
    /// The black side.
    Black,
}

impl Color {
    /// Returns the opposing color.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

/// The kind of a chess piece, independent of its color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PieceKind {
    /// A pawn.
    Pawn,
    /// A knight.
    Knight,
    /// A bishop.
    Bishop,
    /// A rook.
    Rook,
    /// A queen.
    Queen,
    /// A king.
    King,
}

/// A chess piece with its kind and color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}

impl Piece {
    /// Creates a piece of `kind` belonging to `color`.
    #[must_use]
    pub const fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }

    /// Returns the piece's color.
    #[must_use]
    pub const fn color(self) -> Color {
        self.color
    }

    /// Returns the piece's kind.
    #[must_use]
    pub const fn kind(self) -> PieceKind {
        self.kind
    }
}
