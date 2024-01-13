use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::chess_piece::*;

impl ChessPiece for Rook {
    fn piece_type(&self) -> PieceType {
        PieceType::Rook
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
