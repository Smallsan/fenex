use crate::chess_board::coordinates::Coordinates;
use std::convert::TryInto;

/// Represents the notation of a position on a chess board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Notation {
    /// The file (column) of the position.
    pub file: char,
    /// The rank (row) of the position.
    pub rank: char,
}

impl Notation {
    /// Creates a new `Notation` instance.
    /// Returns `None` if the file is not a lowercase ASCII letter or the rank is not a digit.
    pub fn new(file: char, rank: char) -> Option<Notation> {
        if file.is_ascii_lowercase() && rank.is_digit(10) {
            Some(Notation { file, rank })
        } else {
            None
        }
    }
    /// Creates a `Notation` instance from a `Coordinates` instance.
    /// Returns an error if the coordinates are not valid.
    pub fn from_coordinates(coordinates: Coordinates) -> Result<Notation, &'static str> {
        let file = match coordinates.x {
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
        let rank = match coordinates.y {
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
    /// Converts the `Notation` instance to a `Coordinates` instance.
    /// Returns an error if the notation is not valid.
    pub fn to_coordinates(&self) -> Result<Coordinates, &'static str> {
        let x = match self.file {
            'a'..='h' => (self.file as u8 - 'a' as u8) as usize,
            _ => return Err("Invalid file"),
        };
        let y = match self.rank {
            '1'..='8' => (self.rank as u8 - '1' as u8) as usize,
            _ => return Err("Invalid rank"),
        };
        Ok(Coordinates {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        })
    }
    /// Converts the `Notation` instance to a string representation.
    pub fn to_string(&self) -> String {
        format!("{}{}", self.file, self.rank)
    }
    /// Creates a `Notation` instance from a string representation.
    /// Returns `None` if the string is not a valid representation.
    pub fn from_string(input: &str) -> Option<Notation> {
        let chars: Vec<char> = input.chars().collect();
        if chars.len() == 2 {
            Notation::new(chars[0], chars[1])
        } else {
            None
        }
    }
    /// Creates a `Notation` instance from a file and a rank.
    /// Returns `None` if the file is not a lowercase ASCII letter or the rank is not a digit.
    pub fn from_char(file: char, rank: char) -> Option<Notation> {
        Notation::new(file, rank)
    }
}
