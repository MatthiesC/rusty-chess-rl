use crate::{Board, Color};

/// A chess position containing piece placement and the side to move.
///
/// Additional game-state information such as castling rights, en passant, and move counters is
/// intentionally omitted until those rules are implemented. Its [`std::str::FromStr`] and
/// [`std::fmt::Display`] implementations parse and serialize the corresponding two-field FEN
/// subset.
///
/// # Examples
///
/// ```
/// use chess_core::{Color, Position};
///
/// let position: Position = Position::starting_position();
///
/// assert_eq!(position.side_to_move(), Color::White);
/// assert_eq!(
///     position.to_string(),
///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w"
/// );
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    board: Board,
    side_to_move: Color,
}

impl Position {
    /// Creates a position from a board and the side that moves next.
    #[must_use]
    pub const fn new(board: Board, side_to_move: Color) -> Self {
        Self {
            board,
            side_to_move,
        }
    }

    /// Creates the standard starting position with white to move.
    #[must_use]
    pub fn starting_position() -> Self {
        Self::new(Board::starting_position(), Color::White)
    }

    /// Returns the position's board.
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    /// Returns a mutable reference to the position's board.
    ///
    /// Mutating the board does not change the side to move.
    pub const fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Returns the side that moves next.
    #[must_use]
    pub const fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Sets the side that moves next without changing the board.
    pub const fn set_side_to_move(&mut self, side_to_move: Color) {
        self.side_to_move = side_to_move;
    }
}
