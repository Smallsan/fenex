use crate::chess_board::board::Board;
use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::piece::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rook {
    pub color: Color,
    pub coordinates: Coordinates,
}

impl Rook {
    pub fn new(color: Color, coordinates: Coordinates) -> Rook {
        Rook { color, coordinates }
    }
    pub fn can_move_horizontally(&self) -> bool {
        true
    }
}

impl ChessPiece for Rook {
    /// Changes the color of this piece.
    fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Changes the coordinates of this piece.
    fn change_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Rook
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
