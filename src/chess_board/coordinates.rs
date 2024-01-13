use crate::chess_board::notation::Notation;
/// Represents the coordinates on a chess board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    /// The x-coordinate (file).
    pub x: i8,
    /// The y-coordinate (rank).
    pub y: i8,
}

impl Coordinates {
    /// Creates a new `Coordinates` instance.
    pub fn new(x: i8, y: i8) -> Coordinates {
        Coordinates { x, y }
    }

    /// Creates a `Coordinates` instance from a string representation.
    /// Returns `None` if the string is not a valid representation.
    pub fn from_string(input: &str) -> Option<Coordinates> {
        let chars: Vec<char> = input.chars().collect();
        let x = match chars[0] {
            'a'..='h' => (chars[0] as u8 - 'a' as u8) as i8,
            _ => return None,
        };
        let y = match chars[1] {
            '1'..='8' => (chars[1] as u8 - '1' as u8) as i8,
            _ => return None,
        };
        Some(Coordinates { x, y })
    }
    /// Converts the `Coordinates` instance to a string representation.
    pub fn to_string(&self) -> String {
        format!("{}{}", self.x, self.y)
    }
    /// Creates a `Coordinates` instance from a `Notation` instance.
    /// Returns an error if the notation is not valid.
    pub fn from_notation(notation: Notation) -> Result<Coordinates, &'static str> {
        let x = match notation.file {
            'a'..='h' => (notation.file as u8 - 'a' as u8) as i8,
            _ => return Err("Invalid file"),
        };
        let y = match notation.rank {
            '1'..='8' => (notation.rank as u8 - '1' as u8) as i8,
            _ => return Err("Invalid rank"),
        };
        Ok(Coordinates { x, y })
    }
    /// Converts the `Coordinates` instance to a `Notation` instance.
    /// Returns an error if the coordinates are not valid.
    pub fn to_notation(&self) -> Result<Notation, &'static str> {
        let file = match self.x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => return Err("Invalid x coordinate"),
        };
        let rank = match self.y {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => return Err("Invalid y coordinate"),
        };
        Ok(Notation { file, rank })
    }

    /// Converts the `Coordinates` instance to an index in a 1D array representation of the board.
    pub fn to_index(&self) -> usize {
        (self.y * 8 + self.x) as usize
    }

    /// Creates a `Coordinates` instance from an index in a 1D array representation of the board.
    pub fn from_index(index: usize) -> Coordinates {
        Coordinates {
            x: (index % 8) as i8,
            y: (index / 8) as i8,
        }
    }

    /// Checks if the `Coordinates` instance represents a valid position on the board.
    pub fn is_valid(&self) -> bool {
        self.x >= 0 && self.x < 8 && self.y >= 0 && self.y < 8
    }

    /// Checks if the `Coordinates` instance is diagonal to another.
    pub fn is_diagonal(&self, other: Coordinates) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx == dy
    }

    /// Checks if the `Coordinates` instance is straight to another.
    pub fn is_straight(&self, other: Coordinates) -> bool {
        self.x == other.x || self.y == other.y
    }

    /// Checks if the `Coordinates` instance is adjacent to another.
    pub fn is_adjacent(&self, other: Coordinates) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx <= 1 && dy <= 1
    }
}
