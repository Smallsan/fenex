use crate::chess::{
    board::{board::Board, coordinates::Coordinates},
    piece::piece::{ChessPiece, Color, PieceType},
    state::movement::Move,
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

    /// Returns the type of this piece (Queen).
    fn piece_type(&self) -> PieceType {
        PieceType::Queen
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
    fn is_valid_move(
        &mut self,
        destination: Coordinates,
        board: &Board,
        filter_check: bool,
    ) -> bool {
        // Create a copy of the board and apply the move.
        if filter_check {
            let mut new_board = board.clone();
            new_board.make_move_unchecked(Move::new(
                self.coordinates(),
                destination,
                self.piece_type(),
            ));

            if new_board.is_king_in_check(self.color) {
                return false;
            }
        }

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
