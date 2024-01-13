use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::chess_piece::*;

impl ChessPiece for Pawn {
    fn piece_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn color(&self) -> Color {
        self.color
    }

    fn move_piece(&self, from: Coordinates, to: Coordinates) -> Result<(), &'static str> {
        Ok(())
    }

    fn can_capture(&self, target: &dyn ChessPiece) -> bool {
        if self.color() != target.color() {
            let dx = (self.location().x - target.location().x).abs();
            let dy = (self.location().y - target.location().y).abs();
            dx == 1 && dy == 1
        } else {
            false
        }
    }

    fn location(&self) -> Coordinates {
        self.coordinates
    }

    fn can_move_to(&self, location: Coordinates) -> bool {
        // Check if the target location is directly in front of the pawn
        let dx = (self.location().x - location.x).abs();
        let dy = (self.location().y - location.y).abs();
        dx == 0 && dy == 1
    }
}
