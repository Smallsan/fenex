use crate::chess_board::board::Board;
use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::piece::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {
    pub color: Color,
    pub coordinates: Coordinates,
    pub has_moved: bool,
}
impl Pawn {
    pub fn new(color: Color, coordinates: Coordinates, has_moved: bool) -> Pawn {
        Pawn {
            color,
            coordinates,
            has_moved,
        }
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
        board.move_piece_with_coordinates(self.coordinates, destination)
    }

    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    fn is_valid_move(&mut self, destination: Coordinates, board: &Board) -> bool {
        // Calculate the difference between the current and target coordinates.
        let dx = (self.coordinates.x - destination.x).abs();
        let dy = (self.coordinates.y - destination.y).abs();

        // A pawn can move forward one square if that square is unoccupied.
        if dx == 0 && dy == 1 {
            if board.get_piece(destination).is_none() {
                self.has_moved = true;
                return true;
            }
        }

        // A pawn can move forward diagonally one square to capture an opponent's piece.
        if dx == 1 && dy == 1 {
            if let Some(piece) = board.get_piece(destination) {
                if piece.color() != self.color {
                    self.has_moved = true;
                    return true;
                }
            }
        }

        // A pawn can move forward two squares from its starting position if both squares are unoccupied.
        if dx == 0 && dy == 2 {
            let y_between = (self.coordinates.y + destination.y) / 2;
            let between = Coordinates::new(self.coordinates.x, y_between);
            if !self.has_moved
                && ((self.color == Color::White && self.coordinates.y == 2)
                    || (self.color == Color::Black && self.coordinates.y == 7))
                && board.get_piece(destination).is_none()
                && board.get_piece(between).is_none()
            {
                self.has_moved = true;
                return true;
            }
        }

        false
    }
}
