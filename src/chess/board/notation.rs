use std::convert::TryInto;

use super::coordinates::Coordinates;

/// Algebraic chess notation (e.g., "e4", "a1")
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Notation {
    /// File (a-h)
    pub file: char,
    /// Rank (1-8)
    pub rank: char,
}

impl Notation {
    /// Creates notation from file and rank
    pub fn new(file: char, rank: char) -> Result<Notation, &'static str> {
        if !file.is_ascii_lowercase() {
            return Err("File should be a lowercase ASCII letter");
        }
        if !rank.is_digit(10) {
            return Err("Rank should be a digit");
        }
        Ok(Notation { file, rank })
    }

    /// Converts coordinate string to notation
    pub fn from_coordinates_string(input: &str) -> Result<Notation, &'static str> {
        let coordinates = Coordinates::from_string(input)?;
        Notation::from_coordinates(coordinates)
    }

    /// Converts coordinates to algebraic notation
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
