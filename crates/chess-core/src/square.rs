/// A square on a chess board.
///
/// Files and ranks are zero-based: file `0` is `a`, rank `0` is rank `1`, and square index `0` is `a1`. A `Square`
/// always contains a valid board index.
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
        if index < Self::COUNT { Some(Self(index)) } else { None }
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
