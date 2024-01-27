use crate::chess::{board::{board::Board, coordinates::Coordinates}, piece::piece::{ChessPiece, Color, PieceType}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knight {
    pub color: Color,
    pub coordinates: Coordinates,
}

impl Knight {
    pub fn new(color: Color, coordinates: Coordinates) -> Knight {
        Knight { color, coordinates }
    }
}
impl ChessPiece for Knight {
    /// Changes the color of this piece.
    fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Changes the coordinates of this piece.
    fn change_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Knight
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

        // A knight can move in an 'L' shape, which means two squares in one direction
        // and one square in a perpendicular direction.
        if !(dx == 2 && dy == 1) && !(dx == 1 && dy == 2) {
            return false;
        }

        // Check if the destination square is occupied by a piece of the same color.
        if let Some(piece) = board.get_piece(destination) {
            if piece.color() == self.color {
                return false;
            }
        }

        true
    }
}
