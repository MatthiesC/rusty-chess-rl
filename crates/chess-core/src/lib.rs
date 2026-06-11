//! Core chess rules and board representation.
//!
//! The crate uses strong domain types and keeps the engine deterministic. Board squares are indexed
//! from `a1` to `h8`.

mod board;
mod fen;
mod r#move;
mod piece;
mod position;
mod square;

pub use board::Board;
pub use fen::FenError;
pub use r#move::{Move, MoveError, MoveOutcome};
pub use piece::{Color, Piece, PieceKind};
pub use position::Position;
pub use square::Square;
