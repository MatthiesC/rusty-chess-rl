//! Parsing and serialization for the supported Forsyth-Edwards Notation subset.
//!
//! This module handles exactly two fields: piece placement and active color. Castling rights,
//! en passant, and move counters are intentionally rejected until the corresponding engine state
//! exists.

use std::{fmt, str::FromStr};

use crate::{Board, Color, Piece, PieceKind, Position, Square};

/// An error encountered while parsing the supported two-field FEN subset.
///
/// Parsing validates the number and width of ranks, all piece and empty-square symbols, and the
/// active-color field. It does not validate whether the represented position is legal chess.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FenError {
    /// The FEN does not contain exactly piece placement and active color fields.
    InvalidFieldCount {
        /// The number of fields found.
        found: usize,
    },
    /// The piece-placement field does not contain exactly eight ranks.
    InvalidRankCount {
        /// The number of ranks found.
        found: usize,
    },
    /// A rank does not describe exactly eight squares.
    InvalidRankWidth {
        /// The chess rank whose width is invalid.
        rank: u8,
        /// The number of squares described before the error was detected.
        width: u8,
    },
    /// A piece-placement symbol is not valid FEN.
    InvalidPiece(char),
    /// An empty-square count is not valid FEN.
    InvalidEmptyCount(char),
    /// A rank contains adjacent empty-square counts.
    AdjacentEmptyCounts {
        /// The chess rank containing the adjacent counts.
        rank: u8,
    },
    /// The active-color field is neither `w` nor `b`.
    InvalidActiveColor(String),
}

impl fmt::Display for FenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFieldCount { found } => {
                write!(formatter, "expected 2 FEN fields, found {found}")
            }
            Self::InvalidRankCount { found } => {
                write!(formatter, "expected 8 FEN ranks, found {found}")
            }
            Self::InvalidRankWidth { rank, width } => {
                write!(
                    formatter,
                    "FEN rank {rank} describes {width} squares instead of 8"
                )
            }
            Self::InvalidPiece(symbol) => write!(formatter, "invalid FEN piece symbol: {symbol}"),
            Self::InvalidEmptyCount(symbol) => {
                write!(formatter, "invalid FEN empty-square count: {symbol}")
            }
            Self::AdjacentEmptyCounts { rank } => {
                write!(
                    formatter,
                    "FEN rank {rank} contains adjacent empty-square counts"
                )
            }
            Self::InvalidActiveColor(color) => {
                write!(formatter, "invalid FEN active color: {color}")
            }
        }
    }
}

impl std::error::Error for FenError {}

/// Parses piece placement and active color from the supported FEN subset.
///
/// Leading, trailing, and repeated whitespace between the two fields is accepted. Any additional
/// FEN fields are rejected with [`FenError::InvalidFieldCount`].
///
/// # Examples
///
/// ```
/// use chess_core::{Color, Position};
///
/// let position: Position = "8/8/8/8/8/8/8/K6k b".parse()?;
///
/// assert_eq!(position.side_to_move(), Color::Black);
/// # Ok::<(), chess_core::FenError>(())
/// ```
impl FromStr for Position {
    type Err = FenError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = fen.split_whitespace().collect();
        if fields.len() != 2 {
            return Err(FenError::InvalidFieldCount {
                found: fields.len(),
            });
        }

        let board: Board = parse_board(fields[0])?;
        let side_to_move: Color = parse_active_color(fields[1])?;

        Ok(Self::new(board, side_to_move))
    }
}

/// Serializes a position as canonical piece placement followed by active color.
///
/// Consecutive empty squares are compressed into one digit, and ranks are written from rank 8 to
/// rank 1.
impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_board(formatter, self.board())?;
        formatter.write_str(" ")?;
        formatter.write_str(match self.side_to_move() {
            Color::White => "w",
            Color::Black => "b",
        })
    }
}

/// Parses the piece-placement field into a board.
///
/// Each rank must describe exactly eight squares, and empty-square counts must use one digit from
/// `1` through `8`.
fn parse_board(placement: &str) -> Result<Board, FenError> {
    let ranks: Vec<&str> = placement.split('/').collect();
    if ranks.len() != Square::RANK_COUNT as usize {
        return Err(FenError::InvalidRankCount { found: ranks.len() });
    }

    let mut board: Board = Board::empty();

    for (rank_index, rank_text) in ranks.into_iter().enumerate() {
        let rank: u8 = Square::RANK_COUNT - 1 - rank_index as u8;
        let chess_rank: u8 = rank + 1;
        let mut file: u8 = 0;
        let mut previous_was_empty_count: bool = false;

        for symbol in rank_text.chars() {
            if symbol.is_ascii_digit() {
                if previous_was_empty_count {
                    return Err(FenError::AdjacentEmptyCounts { rank: chess_rank });
                }

                let empty_count: u8 = parse_empty_count(symbol)?;
                file += empty_count;
                previous_was_empty_count = true;
            } else {
                if file >= Square::FILE_COUNT {
                    return Err(FenError::InvalidRankWidth {
                        rank: chess_rank,
                        width: file + 1,
                    });
                }

                let piece: Piece = parse_piece(symbol)?;
                let square: Square =
                    Square::new(file, rank).expect("validated FEN coordinates are on the board");
                board.set_piece(square, Some(piece));
                file += 1;
                previous_was_empty_count = false;
            }

            if file > Square::FILE_COUNT {
                return Err(FenError::InvalidRankWidth {
                    rank: chess_rank,
                    width: file,
                });
            }
        }

        if file != Square::FILE_COUNT {
            return Err(FenError::InvalidRankWidth {
                rank: chess_rank,
                width: file,
            });
        }
    }

    Ok(board)
}

/// Parses the active-color field.
fn parse_active_color(active_color: &str) -> Result<Color, FenError> {
    match active_color {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(FenError::InvalidActiveColor(active_color.to_owned())),
    }
}

/// Converts a valid FEN empty-square digit into its numeric count.
fn parse_empty_count(symbol: char) -> Result<u8, FenError> {
    match symbol {
        '1'..='8' => Ok(symbol as u8 - b'0'),
        _ => Err(FenError::InvalidEmptyCount(symbol)),
    }
}

/// Converts a FEN piece symbol into a strongly typed piece.
///
/// Uppercase symbols represent white pieces and lowercase symbols represent black pieces.
fn parse_piece(symbol: char) -> Result<Piece, FenError> {
    let piece: Piece = match symbol {
        'P' => Piece::new(Color::White, PieceKind::Pawn),
        'N' => Piece::new(Color::White, PieceKind::Knight),
        'B' => Piece::new(Color::White, PieceKind::Bishop),
        'R' => Piece::new(Color::White, PieceKind::Rook),
        'Q' => Piece::new(Color::White, PieceKind::Queen),
        'K' => Piece::new(Color::White, PieceKind::King),
        'p' => Piece::new(Color::Black, PieceKind::Pawn),
        'n' => Piece::new(Color::Black, PieceKind::Knight),
        'b' => Piece::new(Color::Black, PieceKind::Bishop),
        'r' => Piece::new(Color::Black, PieceKind::Rook),
        'q' => Piece::new(Color::Black, PieceKind::Queen),
        'k' => Piece::new(Color::Black, PieceKind::King),
        _ => return Err(FenError::InvalidPiece(symbol)),
    };

    Ok(piece)
}

/// Writes canonical FEN piece placement for a board.
fn write_board(formatter: &mut fmt::Formatter<'_>, board: &Board) -> fmt::Result {
    for rank in (0..Square::RANK_COUNT).rev() {
        let mut empty_count: u8 = 0;

        for file in 0..Square::FILE_COUNT {
            let square: Square =
                Square::new(file, rank).expect("board coordinates always form valid squares");

            if let Some(piece) = board.piece_at(square) {
                if empty_count > 0 {
                    write!(formatter, "{empty_count}")?;
                    empty_count = 0;
                }
                write!(formatter, "{}", piece.symbol())?;
            } else {
                empty_count += 1;
            }
        }

        if empty_count > 0 {
            write!(formatter, "{empty_count}")?;
        }
        if rank > 0 {
            formatter.write_str("/")?;
        }
    }

    Ok(())
}
