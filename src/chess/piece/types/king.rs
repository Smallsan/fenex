use crate::chess::{
    board::{board::Board, coordinates::Coordinates},
    piece::piece::{ChessPiece, Color, PieceType},
    state::movement::Move,
};

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
        board.move_piece_with_coordinates(self.coordinates, destination)
    }

    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    fn is_valid_move(&mut self, destination: Coordinates, board: &Board) -> bool {
        // Create a copy of the board and apply the move.
        let mut new_board = board.clone();
        new_board.make_move_unchecked(Move::new(
            self.coordinates(),
            destination,
            self.piece_type(),
        ));

        if new_board.is_king_in_check(self.color) {
            return false;
        }

        // Calculate the difference between the current and target coordinates.
        let dx = (self.coordinates.x - destination.x).abs();
        let dy = (self.coordinates.y - destination.y).abs();

        // A king can move one square in any direction, so the absolute difference
        // between the x and y coordinates should be at most 1.
        if dx > 1 || dy > 1 {
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
