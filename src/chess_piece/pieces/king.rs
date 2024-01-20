use crate::chess_board::board::Board;
use crate::chess_board::coordinates::Coordinates;
use crate::chess_piece::piece::*;

/// Represents a king in a game of chess.
///
/// A king can move one square in any direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct King {
    /// The color of the king.
    pub color: Color,
    /// The current coordinates of the king on the board.
    pub coordinates: Coordinates,
}

impl King {
    /// Creates a new king with the given color and coordinates.
    pub fn new(color: Color, coordinates: Coordinates) -> King {
        King { color, coordinates }
    }

    /// Checks if the king can move to the given coordinates.
    ///
    /// A king can move one square in any direction.
    pub fn can_move_one_square(&self, to: Coordinates) -> bool {
        // Check if the move is one square in any direction
        (self.coordinates.x - to.x).abs() <= 1 && (self.coordinates.y - to.y).abs() <= 1
    }

    pub fn can_castle(&self) -> bool {
        true
    }
    pub fn can_be_checked(&self) -> bool {
        true
    }
    pub fn can_be_checkmated(&self) -> bool {
        true
    }
    pub fn can_be_stalemated(&self) -> bool {
        true
    }
}

impl ChessPiece for King {
    /// Changes the color of this piece.
    fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Changes the coordinates of this piece.
    fn change_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    /// Returns the type of this piece (King).
    fn piece_type(&self) -> PieceType {
        PieceType::King
    }

    /// Returns the color of this piece.
    fn color(&self) -> Color {
        self.color
    }

    /// Moves this piece to the given coordinates.
    ///
    /// If the move is not valid for this piece (i.e., not a one-square move),
    /// returns an error.
    fn move_to(&self, destination: Coordinates, board: &mut Board) -> Result<(), &'static str> {
        if !self.is_valid_move(destination, board) {
            return Err("Invalid move for king");
        }
        Ok(())
    }

    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    fn is_valid_move(&self, location: Coordinates, board: &Board) -> bool {
        true
    }
}
