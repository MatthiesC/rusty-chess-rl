use std::fmt;

use crate::{Color, Move, MoveError, MoveOutcome, Piece, PieceKind, Square};

/// A chess board containing at most one piece on each of its 64 squares.
///
/// The board stores only piece placement. Turn, castling rights, en passant, and move counters
/// belong to a future game-state type.
///
/// Its [`fmt::Display`] representation renders an ASCII board with uppercase white pieces,
/// lowercase black pieces, and `.` for empty squares.
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

    /// Applies a basic move without validating chess movement rules.
    ///
    /// The source piece replaces any piece at the destination. Moving a piece to its current
    /// square succeeds without changing the board. The board is unchanged if the source square
    /// is empty.
    ///
    /// # Errors
    ///
    /// Returns [`MoveError::EmptySource`] if the move's source contains no piece.
    pub fn apply_move(&mut self, chess_move: Move) -> Result<MoveOutcome, MoveError> {
        let moved = self
            .piece_at(chess_move.from())
            .ok_or(MoveError::EmptySource(chess_move.from()))?;

        if chess_move.from() == chess_move.to() {
            return Ok(MoveOutcome::new(moved, None));
        }

        self.set_piece(chess_move.from(), None);
        let captured = self.set_piece(chess_move.to(), Some(moved));

        Ok(MoveOutcome::new(moved, captured))
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

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..Square::RANK_COUNT).rev() {
            write!(formatter, "{} ", rank + 1)?;

            for file in 0..Square::FILE_COUNT {
                if file > 0 {
                    formatter.write_str(" ")?;
                }

                let square =
                    Square::new(file, rank).expect("board coordinates always form valid squares");
                let symbol = self.piece_at(square).map_or('.', piece_symbol);
                write!(formatter, "{symbol}")?;
            }

            formatter.write_str("\n")?;
        }

        formatter.write_str("  a b c d e f g h")
    }
}

const fn piece_symbol(piece: Piece) -> char {
    match (piece.color(), piece.kind()) {
        (Color::White, PieceKind::Pawn) => 'P',
        (Color::White, PieceKind::Knight) => 'N',
        (Color::White, PieceKind::Bishop) => 'B',
        (Color::White, PieceKind::Rook) => 'R',
        (Color::White, PieceKind::Queen) => 'Q',
        (Color::White, PieceKind::King) => 'K',
        (Color::Black, PieceKind::Pawn) => 'p',
        (Color::Black, PieceKind::Knight) => 'n',
        (Color::Black, PieceKind::Bishop) => 'b',
        (Color::Black, PieceKind::Rook) => 'r',
        (Color::Black, PieceKind::Queen) => 'q',
        (Color::Black, PieceKind::King) => 'k',
    }
}
