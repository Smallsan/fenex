use crate::chess_board::coordinates::Coordinates;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {
    pub color: Color,
    pub coordinates: Coordinates,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knight {
    pub color: Color,
    pub coordinates: Coordinates,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bishop {
    pub color: Color,
    pub coordinates: Coordinates,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rook {
    pub color: Color,
    pub coordinates: Coordinates,
}#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Queen {
    pub color: Color,
    pub coordinates: Coordinates,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct King {
    pub color: Color,
    pub coordinates: Coordinates,
}


pub trait ChessPiece {
    fn piece_type(&self) -> PieceType;
    fn color(&self) -> Color;
    fn location(&self) -> Coordinates;
    fn move_piece(&self, from: Coordinates, to: Coordinates) -> Result<(), &'static str>;
    fn can_capture(&self, target: &dyn ChessPiece) -> bool;
    fn can_move_to(&self, location: Coordinates) -> bool;
}

impl Pawn {
    pub fn new(color: Color, coordinates: Coordinates) -> Pawn {
        Pawn { color, coordinates}
    }
    pub fn can_move_forward(&self) -> bool {
        true
    }
    pub fn can_move_two_squares(&self) -> bool {
        true
    }
    pub fn can_capture_diagonally(&self) -> bool {
        true
    }
    pub fn can_be_promoted(&self) -> bool {
        true
    }
    pub fn can_be_en_passanted(&self) -> bool {
        true
    }
    pub fn can_be_captured_en_passant(&self) -> bool {
        true
    }
}

impl Knight {
    pub fn new(color: Color, coordinates: Coordinates) -> Knight {
        Knight { color, coordinates }
    }
    pub fn can_jump_over_pieces(&self) -> bool {
        true
    }
    pub fn can_move_in_l_shape(&self) -> bool {
        true
    }
}

impl Bishop {
    pub fn new(color: Color, coordinates: Coordinates) -> Bishop {
        Bishop { color, coordinates }
    }
    pub fn can_move_diagonally(&self) -> bool {
        true
    }
}

impl Rook {
    pub fn new(color: Color, coordinates: Coordinates) -> Rook {
        Rook { color, coordinates }
    }
    pub fn can_move_horizontally(&self) -> bool {
        true
    }
}

impl Queen {
    pub fn new(color: Color, coordinates: Coordinates) -> Queen {
        Queen { color, coordinates }
    }
    pub fn can_move_horizontally_and_vertically(&self) -> bool {
        true
    }
}

impl King {
    pub fn new(color: Color, coordinates: Coordinates) -> King {
        King { color, coordinates }
    }
    pub fn can_move_one_square(&self) -> bool {
        true
    }
    pub fn can_castle(&self) -> bool {
        true
    }
    pub fn can_be_checked(&self) -> bool {
        true
    }
    pub fn can_be_checkmated(&self) -> bool {
        true
    }
    pub fn can_be_stalemated(&self) -> bool {
        true
    }
}





