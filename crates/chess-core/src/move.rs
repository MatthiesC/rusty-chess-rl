//! Basic move descriptions, application results, and errors.
//!
//! These types describe rule-agnostic piece relocation. Chess-rule validation and move generation
//! will be layered on top separately.

use std::fmt;

use crate::{Piece, Square};

/// A request to move a piece from one square to another.
///
/// A move describes only its source and destination. It does not imply that the move follows
/// chess rules.
///
/// # Examples
///
/// ```
/// use chess_core::{Move, Square};
///
/// let e2: Square = Square::new(4, 1).expect("e2 is on the board");
/// let e4: Square = Square::new(4, 3).expect("e4 is on the board");
/// let chess_move: Move = Move::new(e2, e4);
///
/// assert_eq!(chess_move.from(), e2);
/// assert_eq!(chess_move.to(), e4);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    from: Square,
    to: Square,
}

impl Move {
    /// Creates a move from `from` to `to`.
    #[must_use]
    pub const fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }

    /// Returns the source square.
    #[must_use]
    pub const fn from(self) -> Square {
        self.from
    }

    /// Returns the destination square.
    #[must_use]
    pub const fn to(self) -> Square {
        self.to
    }
}

/// The result of successfully applying a basic move to a board.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MoveOutcome {
    moved: Piece,
    captured: Option<Piece>,
}

impl MoveOutcome {
    /// Creates an outcome for a moved piece and an optional captured piece.
    pub(crate) const fn new(moved: Piece, captured: Option<Piece>) -> Self {
        Self { moved, captured }
    }

    /// Returns the piece that was moved.
    #[must_use]
    pub const fn moved(self) -> Piece {
        self.moved
    }

    /// Returns the piece replaced at the destination, if any.
    #[must_use]
    pub const fn captured(self) -> Option<Piece> {
        self.captured
    }
}

/// An error that prevents a basic move from being applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveError {
    /// The move's source square contains no piece.
    EmptySource(Square),
}

impl fmt::Display for MoveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySource(square) => {
                write!(formatter, "source square {} is empty", square.index())
            }
        }
    }
}

impl std::error::Error for MoveError {}
