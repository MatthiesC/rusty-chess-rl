/// A square on a chess board.
///
/// Files and ranks are zero-based: file `0` is `a`, rank `0` is rank `1`, and
/// square index `0` is `a1`. A `Square` always contains a valid board index.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Square(u8);

impl Square {
    /// The number of files on a chess board.
    pub const FILE_COUNT: u8 = 8;

    /// The number of ranks on a chess board.
    pub const RANK_COUNT: u8 = 8;

    /// The number of squares on a chess board.
    pub const COUNT: u8 = Self::FILE_COUNT * Self::RANK_COUNT;

    /// Creates a square from a zero-based file and rank.
    ///
    /// Returns `None` if either coordinate is outside the board.
    #[must_use]
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file < Self::FILE_COUNT && rank < Self::RANK_COUNT {
            Some(Self(rank * Self::FILE_COUNT + file))
        } else {
            None
        }
    }

    /// Creates a square from an index in the range `0..64`.
    ///
    /// Returns `None` if `index` is outside the board.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < Self::COUNT {
            Some(Self(index))
        } else {
            None
        }
    }

    /// Returns the square's zero-based file.
    #[must_use]
    pub const fn file(self) -> u8 {
        self.0 % Self::FILE_COUNT
    }

    /// Returns the square's zero-based rank.
    #[must_use]
    pub const fn rank(self) -> u8 {
        self.0 / Self::FILE_COUNT
    }

    /// Returns the square's zero-based board index.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_squares() {
        let a1 = Square::new(0, 0).expect("a1 is valid");
        let h8 = Square::new(7, 7).expect("h8 is valid");

        assert_eq!(a1.index(), 0);
        assert_eq!(h8.index(), 63);
        assert_eq!((h8.file(), h8.rank()), (7, 7));
    }

    #[test]
    fn rejects_out_of_bounds_coordinates_and_indices() {
        assert_eq!(Square::new(8, 0), None);
        assert_eq!(Square::new(0, 8), None);
        assert_eq!(Square::from_index(64), None);
    }

    #[test]
    fn index_round_trip_preserves_every_square() {
        for index in 0..Square::COUNT {
            let square = Square::from_index(index).expect("index is on the board");

            assert_eq!(Square::new(square.file(), square.rank()), Some(square));
        }
    }
}
