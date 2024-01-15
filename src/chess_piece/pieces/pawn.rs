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

    fn move_piece(&mut self, to: Coordinates) -> Result<(), &'static str> {
        Ok(())
    }

    fn can_capture(&self, target: &dyn ChessPiece) -> bool {
        if self.color() != target.color() {
            let dx = (self.coordinates().x - target.coordinates().x).abs();
            let dy = (self.coordinates().y - target.coordinates().y).abs();
            dx == 1 && dy == 1
        } else {
            false
        }
    }

    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    fn can_move_to(&self, coordinates: Coordinates) -> bool {
        // Check if the target location is directly in front of the pawn
        let dx = (self.coordinates().x - coordinates.x).abs();
        let dy = (self.coordinates().y - coordinates.y).abs();
        dx == 0 && dy == 1
    }
}
