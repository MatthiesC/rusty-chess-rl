//! Python bindings for the Rust chess engine.
//!
//! The binding layer is currently a scaffold. It will eventually expose the core engine through a
//! small environment-style Python API.

/// Adds two unsigned integers.
///
/// This placeholder function will be removed when the first PyO3 bindings are introduced.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
