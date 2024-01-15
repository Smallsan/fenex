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
    fn piece_type(&self) -> PieceType {
        PieceType::Rook
    }

    fn color(&self) -> Color {
        self.color
    }

    fn move_piece(&mut self, to: Coordinates) -> Result<(), &'static str> {
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
