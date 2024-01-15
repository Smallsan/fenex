use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::piece::*;

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
    /// Changes the color of this piece.
    fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Changes the coordinates of this piece.
    fn change_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Queen
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

    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    fn can_move_to(&self, coordinates: Coordinates) -> bool {
        true
    }
}
