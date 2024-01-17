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
    fn move_piece(&mut self, to: Coordinates) -> Result<(), &'static str> {
        if self.can_move_to(to) {
            self.coordinates = to;
            Ok(())
        } else {
            Err("Invalid move")
        }
    }

    /// Returns whether this piece can capture the target piece.
    ///
    /// A piece can capture another piece if they are of different colors.
    fn can_capture(&self, target: &dyn ChessPiece) -> bool {
        self.color != target.color()
    }

    /// Returns the current location of this piece.
    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    /// Returns whether this piece can move to the given coordinates.
    ///
    /// A bishop can move to a location if it is a diagonal move.
    fn can_move_to(&self, to: Coordinates) -> bool {
        self.coordinates.is_diagonal(to)
    }
}
