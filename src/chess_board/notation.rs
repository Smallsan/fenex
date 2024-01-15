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
    pub fn from_coordinates(coordinates: Coordinates) -> Result<Notation, &'static str> {
        let file = match coordinates.x {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => return Err("Invalid x coordinate for notation conversion"),
        };
        let rank = match coordinates.y {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            _ => return Err("Invalid y coordinate for notation conversion"),
        };
        Ok(Notation { file, rank })
    }

    pub fn to_coordinates(&self) -> Result<Coordinates, &'static str> {
        let x = match self.file {
            'a'..='h' => (self.file as u8 - 'a' as u8 + 1) as usize,
            _ => return Err("Invalid file for coordinate conversion"),
        };
        let y = match self.rank {
            '1'..='8' => (self.rank as u8 - '1' as u8 + 1) as usize,
            _ => return Err("Invalid rank for coordinate conversion"),
        };
        Ok(Coordinates {
            x: x.try_into()
                .expect("Invalid x coordinate for coordinates conversion"),
            y: y.try_into()
                .expect("Invalid y coordinate for coordinates conversion"),
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
