use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::chess_piece::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct King {
    pub color: Color,
    pub coordinates: Coordinates,
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

impl ChessPiece for King {
    fn piece_type(&self) -> PieceType {
        PieceType::King
    }

    fn color(&self) -> Color {
        self.color
    }

    fn move_piece(&self, from: Coordinates, to: Coordinates) -> Result<(), &'static str> {
        Ok(())
    }

    fn can_capture(&self, target: &dyn ChessPiece) -> bool {
        if self.color != target.color() {
            return true;
        }
        false
    }

    fn location(&self) -> Coordinates {
        self.coordinates
    }

    fn can_move_to(&self, coordinates: Coordinates) -> bool {
        true
    }
}
