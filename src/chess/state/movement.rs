use crate::chess::{
    board::{
        board::{Board, BoardType},
        coordinates::Coordinates,
    },
    piece::piece::{Color, PieceType},
};

/// Represents a move in the game of chess.
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    piece_type: PieceType,
    from: Coordinates,
    to: Coordinates,
}

impl Move {
    /// Creates a new move from the given coordinates.
    pub fn new(from: Coordinates, to: Coordinates, piece_type: PieceType) -> Self {
        Self {
            from,
            to,
            piece_type,
        }
    }

    /// Returns the coordinates of the piece that is being moved.
    pub fn from(&self) -> Coordinates {
        self.from
    }

    /// Returns the coordinates of the destination of the piece.
    pub fn to(&self) -> Coordinates {
        self.to
    }

    /// Returns the type of the piece that is being moved.
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}

impl Board {
    /// Generates all possible moves for a chess color in the board.
    /// Doesn't check for pins and check situations yet
    /// Can't generate En passants and castles yet
    pub fn generate_moves(&self, color: Color, check_for_check: bool) -> Vec<Move> {
        let mut moves = Vec::new();

        match &self.board_type {
            BoardType::OneDimensional(board) => {
                for i in 1..=64 {
                    if let Some(piece) = board[i - 1] {
                        if piece.color() == color {
                            for x_to in 1..=8 {
                                for y_to in 1..=8 {
                                    let destination = Coordinates::new(x_to, y_to);
                                    let piece_type = piece.piece_type();
                                    if piece.is_valid_move(destination, self, check_for_check) {
                                        moves.push(Move::new(
                                            piece.coordinates(),
                                            destination,
                                            piece_type,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            BoardType::TwoDimensional(board) => {
                for x_from in 1..=8 {
                    for y_from in 1..=8 {
                        if let Some(piece) = board[x_from - 1][y_from - 1] {
                            if piece.color() == color {
                                for x_to in 1..=8 {
                                    for y_to in 1..=8 {
                                        let piece_type = piece.piece_type();
                                        let destination = Coordinates::new(x_to, y_to);
                                        if piece.is_valid_move(destination, self, true) {
                                            moves.push(Move::new(
                                                piece.coordinates(),
                                                destination,
                                                piece_type,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }
    /// Makes a move in the board, Without considering legality.
    pub fn make_move_unchecked(&mut self, m: Move) {
        match &mut self.board_type {
            BoardType::OneDimensional(board) => {
                let from_index = m.from.to_index().expect("Invalid from coordinates");
                let to_index = m.to.to_index().expect("Invalid to coordinates");
                board[to_index] = board[from_index].take();
            }
            BoardType::TwoDimensional(board) => {
                let (from_x, from_y) = m.from.to_tuple();
                let (to_x, to_y) = m.to.to_tuple();
                board[to_x][to_y] = board[from_x][from_y].take();
            }
        }
    }
}
