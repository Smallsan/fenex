use crate::chess_board::board::Board;
use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::piece::*;

/// Represents a bishop in a game of chess.
///
/// A bishop can move any number of squares diagonally.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bishop {
    /// The color of the bishop.
    pub color: Color,
    /// The current coordinates of the bishop on the board.
    pub coordinates: Coordinates,
}
impl Bishop {
    /// Creates a new bishop with the given color and coordinates.
    pub fn new(color: Color, coordinates: Coordinates) -> Bishop {
        Bishop { color, coordinates }
    }
}
impl ChessPiece for Bishop {
    /// Changes the color of this piece.
    fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Changes the coordinates of this piece.
    fn change_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    /// Returns the type of this piece.
    fn piece_type(&self) -> PieceType {
        PieceType::Bishop
    }

    /// Returns the color of this piece.
    fn color(&self) -> Color {
        self.color
    }

    /// Moves this piece to the given coordinates.
    ///
    /// If the move is not valid for this piece, returns an error.
    fn move_to(&self, destination: Coordinates, board: &mut Board) -> Result<(), &'static str> {
        board.move_piece_with_coordinates(self.coordinates, destination)
    }

    /// Returns the current location of this piece.
    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    /// Returns whether this piece can move to the given coordinates.
    /// A bishop can move to a location if it is a diagonal move.
    fn is_valid_move(&mut self, destination: Coordinates, board: &Board) -> bool {
        // Calculate the difference between the current and target coordinates.
        let dx = (self.coordinates.x - destination.x).abs();
        let dy = (self.coordinates.y - destination.y).abs();

        // A bishop can move any number of squares diagonally, so the absolute difference
        // between the x and y coordinates should be the same.
        if dx != dy {
            return false;
        }

        // Check if the path to the target coordinates is clear.
        let x_step = if destination.x > self.coordinates.x {
            1
        } else {
            -1
        };
        let y_step = if destination.y > self.coordinates.y {
            1
        } else {
            -1
        };
        let mut x = self.coordinates.x + x_step;
        let mut y = self.coordinates.y + y_step;

        while x != destination.x || y != destination.y {
            if x < 1 || x > 8 || y < 1 || y > 8 {
                return false;
            }
            if board.get_piece(Coordinates::new(x, y)).is_some() {
                return false;
            }
            if x_step > 0 && x == 8 || x_step < 0 && x == 1 {
                return false;
            }
            if y_step > 0 && y == 8 || y_step < 0 && y == 1 {
                return false;
            }
            x += x_step;
            y += y_step;
        }

        // Check the destination square.
        if let Some(target_piece) = board.get_piece(destination) {
            if target_piece.color() == self.color {
                return false;
            }
        }
        true
    }
}
