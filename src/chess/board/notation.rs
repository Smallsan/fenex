use std::convert::TryInto;

use super::coordinates::Coordinates;

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
    pub fn new(file: char, rank: char) -> Result<Notation, &'static str> {
        if !file.is_ascii_lowercase() {
            return Err("File should be a lowercase ASCII letter");
        }
        if !rank.is_digit(10) {
            return Err("Rank should be a digit");
        }
        Ok(Notation { file, rank })
    }

    /// Creates a `Notation` instance from a string representation of coordinates.
    pub fn from_coordinates_string(input: &str) -> Result<Notation, &'static str> {
        let coordinates = Coordinates::from_string(input)?;
        Notation::from_coordinates(coordinates)
    }

    /// Creates a `Notation` instance from a `Coordinates` instance.
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

    /// Converts the `Notation` instance to a `Coordinates` instance.
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
    pub fn from_string(input: &str) -> Result<Notation, &'static str> {
        let chars: Vec<char> = input.chars().collect();
        if chars.len() != 2 {
            return Err("Input should be exactly two characters");
        }
        Notation::new(chars[0], chars[1]).map_err(|_| "Invalid notation")
    }
}
