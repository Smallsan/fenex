use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::piece::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knight {
    pub color: Color,
    pub coordinates: Coordinates,
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
impl ChessPiece for Knight {
    fn piece_type(&self) -> PieceType {
        PieceType::Knight
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