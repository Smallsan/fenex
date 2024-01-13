use crate::chess_board::notation::Notation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    pub x: i8,
    pub y: i8,
}

impl Coordinates {
    pub fn new(x: i8, y: i8) -> Coordinates {
        Coordinates { x, y }
    }
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
    pub fn to_string(&self) -> String {
        format!("{}{}", self.x, self.y)
    }
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
    pub fn to_index(&self) -> usize {
        (self.y * 8 + self.x) as usize
    }
    pub fn from_index(index: usize) -> Coordinates {
        Coordinates {
            x: (index % 8) as i8,
            y: (index / 8) as i8,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.x >= 0 && self.x < 8 && self.y >= 0 && self.y < 8
    }
    pub fn is_diagonal(&self, other: Coordinates) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx == dy
    }
    pub fn is_straight(&self, other: Coordinates) -> bool {
        self.x == other.x || self.y == other.y
    }
    pub fn is_adjacent(&self, other: Coordinates) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx <= 1 && dy <= 1
    }
}
