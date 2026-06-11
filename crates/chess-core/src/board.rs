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

#[cfg(test)]
mod tests {
    use super::*;

    fn square(file: u8, rank: u8) -> Square {
        Square::new(file, rank).expect("test square must be valid")
    }

    #[test]
    fn empty_board_contains_no_pieces() {
        let board = Board::empty();

        assert!(board.iter().all(|(_, piece)| piece.is_none()));
    }

    #[test]
    fn starting_position_has_expected_pieces() {
        let board = Board::starting_position();

        assert_eq!(
            board.piece_at(square(0, 0)),
            Some(Piece::new(Color::White, PieceKind::Rook))
        );
        assert_eq!(
            board.piece_at(square(4, 0)),
            Some(Piece::new(Color::White, PieceKind::King))
        );
        assert_eq!(
            board.piece_at(square(3, 7)),
            Some(Piece::new(Color::Black, PieceKind::Queen))
        );
        assert_eq!(
            board.piece_at(square(7, 7)),
            Some(Piece::new(Color::Black, PieceKind::Rook))
        );
        assert_eq!(board.piece_at(square(4, 3)), None);
    }

    #[test]
    fn starting_position_contains_sixteen_pieces_per_color() {
        let board = Board::starting_position();

        let white_count = board
            .iter()
            .filter(|(_, piece)| piece.is_some_and(|piece| piece.color() == Color::White))
            .count();
        let black_count = board
            .iter()
            .filter(|(_, piece)| piece.is_some_and(|piece| piece.color() == Color::Black))
            .count();

        assert_eq!(white_count, 16);
        assert_eq!(black_count, 16);
    }

    #[test]
    fn set_piece_replaces_and_clears_square() {
        let mut board = Board::empty();
        let e4 = square(4, 3);
        let pawn = Piece::new(Color::White, PieceKind::Pawn);

        assert_eq!(board.set_piece(e4, Some(pawn)), None);
        assert_eq!(board.piece_at(e4), Some(pawn));
        assert_eq!(board.set_piece(e4, None), Some(pawn));
        assert_eq!(board.piece_at(e4), None);
    }

    #[test]
    fn iteration_visits_every_square_in_index_order() {
        let board = Board::empty();
        let indices = board
            .iter()
            .map(|(square, _)| square.index())
            .collect::<Vec<_>>();

        assert_eq!(indices, (0..Square::COUNT).collect::<Vec<_>>());
    }
}
