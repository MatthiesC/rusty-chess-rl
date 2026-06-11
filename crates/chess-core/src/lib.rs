//! Core chess rules and board representation.
//!
//! The crate uses strong domain types and keeps the engine deterministic. Board
//! squares are indexed from `a1` to `h8`.

mod board;
mod piece;
mod square;

pub use board::Board;
pub use piece::{Color, Piece, PieceKind};
pub use square::Square;
