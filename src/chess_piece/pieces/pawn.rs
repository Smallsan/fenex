use crate::chess_board::board::Board;
use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::piece::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {
    pub color: Color,
    pub coordinates: Coordinates,
}
impl Pawn {
    pub fn new(color: Color, coordinates: Coordinates) -> Pawn {
        Pawn { color, coordinates }
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
impl ChessPiece for Pawn {
    /// Changes the color of this piece.
    fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Changes the coordinates of this piece.
    fn change_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn color(&self) -> Color {
        self.color
    }

    fn move_to(&self, destination: Coordinates, board: &mut Board) -> Result<(), &'static str> {
        Ok(())
    }

    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    fn is_valid_move(&self, location: Coordinates, board: &Board) -> bool {
        true
    }
}
