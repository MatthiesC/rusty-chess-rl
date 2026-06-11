use crate::{Color, Piece, PieceKind, Square};

/// A chess board containing at most one piece on each of its 64 squares.
///
/// The board stores only piece placement. Turn, castling rights, en passant,
/// and move counters belong to a future game-state type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    squares: [Option<Piece>; Square::COUNT as usize],
}

impl Board {
    /// Creates a board with no pieces.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            squares: [None; Square::COUNT as usize],
        }
    }

    /// Creates a board with pieces in the standard starting position.
    #[must_use]
    pub fn starting_position() -> Self {
        const BACK_RANK: [PieceKind; Square::FILE_COUNT as usize] = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        let mut board = Self::empty();

        for (file, kind) in (0..Square::FILE_COUNT).zip(BACK_RANK) {
            board.squares[file as usize] = Some(Piece::new(Color::White, kind));
            board.squares[(Square::FILE_COUNT + file) as usize] =
                Some(Piece::new(Color::White, PieceKind::Pawn));
            board.squares[(6 * Square::FILE_COUNT + file) as usize] =
                Some(Piece::new(Color::Black, PieceKind::Pawn));
            board.squares[(7 * Square::FILE_COUNT + file) as usize] =
                Some(Piece::new(Color::Black, kind));
        }

        board
    }

    /// Returns the piece on `square`, or `None` if the square is empty.
    #[must_use]
    pub const fn piece_at(&self, square: Square) -> Option<Piece> {
        self.squares[square.index() as usize]
    }

    /// Replaces the contents of `square` and returns the previous piece.
    ///
    /// Passing `None` clears the square.
    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) -> Option<Piece> {
        std::mem::replace(&mut self.squares[square.index() as usize], piece)
    }

    /// Iterates over every square and its optional piece from `a1` to `h8`.
    pub fn iter(&self) -> impl Iterator<Item = (Square, Option<Piece>)> + '_ {
        self.squares.iter().enumerate().map(|(index, piece)| {
            let square = Square::from_index(index as u8)
                .expect("board storage indices always correspond to valid squares");
            (square, *piece)
        })
    }
}
