use crate::chess::{
    board::{board::Board, coordinates::Coordinates},
    piece::piece::{ChessPiece, Color, PieceType},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Queen {
    pub color: Color,
    pub coordinates: Coordinates,
}

impl Queen {
    pub fn new(color: Color, coordinates: Coordinates) -> Queen {
        Queen { color, coordinates }
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

        // A queen can move any number of squares along a rank, file, or diagonal.
        if dx != dy && dx != 0 && dy != 0 {
            return false;
        }

        // Check if the path to the target coordinates is clear.
        let x_step = if destination.x > self.coordinates.x {
            1
        } else if destination.x < self.coordinates.x {
            -1
        } else {
            0
        };
        let y_step = if destination.y > self.coordinates.y {
            1
        } else if destination.y < self.coordinates.y {
            -1
        } else {
            0
        };
        let mut x = self.coordinates.x + x_step;
        let mut y = self.coordinates.y + y_step;
        while x != destination.x || y != destination.y {
            if let Some(_piece) = board.get_piece(Coordinates::new(x, y)) {
                // There's a piece blocking the path.
                return false;
            }
            x += x_step;
            y += y_step;
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
