use crate::chess::{
    board::{board::Board, coordinates::Coordinates},
    piece::piece::{ChessPiece, Color, PieceType},
    state::movement::Move,
};

/// Represents a Knight chess piece.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knight {
    pub color: Color,
    pub coordinates: Coordinates,
}

impl Knight {
    /// Creates a new Knight with the specified color and coordinates.
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
    
    /// Returns the type of this piece (Knight).
    fn piece_type(&self) -> PieceType {
        PieceType::Knight
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
