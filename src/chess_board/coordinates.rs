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
    /// The string should be in the format "x,y".
    /// Returns `None` if the string is not a valid representation.
    pub fn from_string(input: &str) -> Result<Coordinates, &'static str> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 2 {
            return Err("Input should be in the format 'x,y'");
        }
        let x = parts[0]
            .trim()
            .parse::<i8>()
            .map_err(|_| "Invalid x coordinate")?;
        let y = parts[1]
            .trim()
            .parse::<i8>()
            .map_err(|_| "Invalid y coordinate")?;
        if x < 1 || x > 8 || y < 1 || y > 8 {
            return Err("Coordinates should be between 1 and 8");
        }
        Ok(Coordinates::new(x, y))
    }

    /// Creates a `Coordinates` instance from a string representation of notations.
    pub fn from_notation_string(s: &str) -> Result<Self, &'static str> {
        if s.len() != 2 {
            return Err("Invalid notation string");
        }

        let file = s.chars().nth(0).unwrap().to_ascii_lowercase();
        let rank = s.chars().nth(1).unwrap().to_digit(10).unwrap();

        if file < 'a' || file > 'h' || rank < 1 || rank > 8 {
            return Err("Invalid notation string");
        }

        let x = file as i8 - 'a' as i8 + 1;
        let y = rank as i8;

        Ok(Self { x, y })
    }
    /// Converts the `Coordinates` instance to a string representation.
    pub fn to_string(&self) -> String {
        format!("{}{}", self.x + 1, self.y + 1)
    }

    /// Creates a `Coordinates` instance from a `Notation` instance.
    /// Returns an error if the notation is not valid.
    pub fn from_notation(notation: Notation) -> Result<Coordinates, &'static str> {
        let x = match notation.file {
            'a'..='h' => (notation.file as u8 - 'a' as u8 + 1) as i8,
            _ => return Err("Invalid file"),
        };
        let y = match notation.rank {
            '1'..='8' => (notation.rank as u8 - '1' as u8 + 1) as i8,
            _ => return Err("Invalid rank"),
        };
        Ok(Coordinates::new(x, y))
    }
    /// Converts the `Coordinates` instance to a `Notation` instance.
    /// Returns an error if the coordinates are not valid.
    pub fn to_notation(&self) -> Result<Notation, &'static str> {
        let file = match self.x {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => return Err("Invalid x coordinate"),
        };
        let rank = match self.y {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            _ => return Err("Invalid y coordinate"),
        };
        Ok(Notation { file, rank })
    }

    /// Converts the `Coordinates` instance to an index in a 1D array representation of the board.
    pub fn to_index(&self) -> Result<usize, &'static str> {
        if self.y < 1 || self.y > 8 {
            return Err("y coordinate is out of bounds");
        }
        let x = (self.x - 1) as usize;
        let y = (self.y - 1) as usize;
        Ok(y * 8 + x)
    }

    /// Creates a `Coordinates` instance from an index in a 1D array representation of the board.
    pub fn from_index(index: usize) -> Coordinates {
        Coordinates::new((index % 8 + 1) as i8, (index / 8 + 1) as i8)
    }

    /// Checks if the `Coordinates` instance represents a valid position on the board.
    pub fn is_valid(&self) -> bool {
        self.x >= 1 && self.x <= 8 && self.y >= 1 && self.y <= 8
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
