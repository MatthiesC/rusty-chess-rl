use std::fmt;

use crate::{Piece, Square};

/// A request to move a piece from one square to another.
///
/// A move describes only its source and destination. It does not imply that the move follows chess rules.
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
