use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::chess_piece::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Queen {
    pub color: Color,
    pub coordinates: Coordinates,
}

impl Queen {
    pub fn new(color: Color, coordinates: Coordinates) -> Queen {
        Queen { color, coordinates }
    }
    pub fn can_move_horizontally_and_vertically(&self) -> bool {
        true
    }
}

impl ChessPiece for Queen {
    fn piece_type(&self) -> PieceType {
        PieceType::Queen
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
