use crate::{
    bishop::Bishop,
    coordinates::Coordinates,
    king::King,
    knight::Knight,
    pawn::Pawn,
    piece::{ChessPiece, Color},
    queen::Queen,
    rook::Rook,
};

// The ChessPieceEnum represents the different types of chess pieces.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChessPieceEnum {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl ChessPieceEnum {
    /// Update both color and coordinates of the chess piece.
    pub fn change_color_and_coordinates(&mut self, color: Color, coordinates: Coordinates) {
        match self {
            ChessPieceEnum::Pawn(pawn) => {
                pawn.change_coordinates(coordinates);
                pawn.change_color(color);
            }
            ChessPieceEnum::Knight(king) => {
                king.change_coordinates(coordinates);
                king.change_color(color);
            }
            ChessPieceEnum::Bishop(bishop) => {
                bishop.change_coordinates(coordinates);
                bishop.change_color(color);
            }
            ChessPieceEnum::Rook(rook) => {
                rook.change_coordinates(coordinates);
                rook.change_color(color);
            }
            ChessPieceEnum::Queen(queen) => {
                queen.change_coordinates(coordinates);
                queen.change_color(color);
            }
            ChessPieceEnum::King(king) => {
                king.change_coordinates(coordinates);
                king.change_color(color);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BoardType {
    OneDimensional(Vec<Option<ChessPieceEnum>>),
    TwoDimensional([[Option<ChessPieceEnum>; 8]; 8]),
}

#[derive(Debug, PartialEq)]
pub struct Board {
    board_type: BoardType,
}

impl Board {
    /// Creates a new `1D Board` with all squares empty.
    pub fn new_one_dimensional() -> Self {
        Board {
            board_type: BoardType::OneDimensional(vec![None; 64]),
        }
    }
    /// Creates a new `2D Board` with all squares empty.
    pub fn new_two_dimensional() -> Self {
        Board {
            board_type: BoardType::TwoDimensional([[None; 8]; 8]),
        }
    }

    /// Creates a new `1D Board` with pieces in the starting position.
    pub fn new_one_dimensional_starting_position() -> Self {
        let mut board = vec![None; 64];

        // Set up the pawns.
        for i in 8..16 {
            board[i] = Some(ChessPieceEnum::Pawn(Pawn {
                color: Color::White,
                coordinates: Coordinates::from_index(i),
            }));
            board[i + 40] = Some(ChessPieceEnum::Pawn(Pawn {
                color: Color::Black,
                coordinates: Coordinates::from_index(i + 40),
            }));
        }

        // Set up the rooks.
        board[0] = Some(ChessPieceEnum::Rook(Rook {
            color: Color::White,
            coordinates: Coordinates::from_index(0),
        }));
        board[7] = Some(ChessPieceEnum::Rook(Rook {
            color: Color::White,
            coordinates: Coordinates::from_index(7),
        }));
        board[56] = Some(ChessPieceEnum::Rook(Rook {
            color: Color::Black,
            coordinates: Coordinates::from_index(56),
        }));
        board[63] = Some(ChessPieceEnum::Rook(Rook {
            color: Color::Black,
            coordinates: Coordinates::from_index(63),
        }));

        // Set up the knights.
        board[1] = Some(ChessPieceEnum::Knight(Knight {
            color: Color::White,
            coordinates: Coordinates::from_index(1),
        }));
        board[6] = Some(ChessPieceEnum::Knight(Knight {
            color: Color::White,
            coordinates: Coordinates::from_index(6),
        }));
        board[57] = Some(ChessPieceEnum::Knight(Knight {
            color: Color::Black,
            coordinates: Coordinates::from_index(57),
        }));
        board[62] = Some(ChessPieceEnum::Knight(Knight {
            color: Color::Black,
            coordinates: Coordinates::from_index(62),
        }));

        // Set up the bishops.
        board[2] = Some(ChessPieceEnum::Bishop(Bishop {
            color: Color::White,
            coordinates: Coordinates::from_index(2),
        }));
        board[5] = Some(ChessPieceEnum::Bishop(Bishop {
            color: Color::White,
            coordinates: Coordinates::from_index(5),
        }));
        board[58] = Some(ChessPieceEnum::Bishop(Bishop {
            color: Color::Black,
            coordinates: Coordinates::from_index(58),
        }));
        board[61] = Some(ChessPieceEnum::Bishop(Bishop {
            color: Color::Black,
            coordinates: Coordinates::from_index(61),
        }));

        // Set up the queen and king.
        board[3] = Some(ChessPieceEnum::Queen(Queen {
            color: Color::White,
            coordinates: Coordinates::from_index(3),
        }));
        board[4] = Some(ChessPieceEnum::King(King {
            color: Color::White,
            coordinates: Coordinates::from_index(4),
        }));
        board[59] = Some(ChessPieceEnum::Queen(Queen {
            color: Color::Black,
            coordinates: Coordinates::from_index(59),
        }));
        board[60] = Some(ChessPieceEnum::King(King {
            color: Color::Black,
            coordinates: Coordinates::from_index(60),
        }));

        Board {
            board_type: BoardType::OneDimensional(board),
        }
    }

    /// Creates a new `2D Board` with pieces in the starting position.
    pub fn new_two_dimensional_starting_position() -> Self {
        let mut board = Board::new_two_dimensional();

        let back_rank = [
            ChessPieceEnum::Rook(Rook::new(Color::White, Coordinates::new(1, 1))),
            ChessPieceEnum::Knight(Knight::new(Color::White, Coordinates::new(2, 1))),
            ChessPieceEnum::Bishop(Bishop::new(Color::White, Coordinates::new(3, 1))),
            ChessPieceEnum::Queen(Queen::new(Color::White, Coordinates::new(4, 1))),
            ChessPieceEnum::King(King::new(Color::White, Coordinates::new(5, 1))),
            ChessPieceEnum::Bishop(Bishop::new(Color::White, Coordinates::new(6, 1))),
            ChessPieceEnum::Knight(Knight::new(Color::White, Coordinates::new(7, 1))),
            ChessPieceEnum::Rook(Rook::new(Color::White, Coordinates::new(8, 1))),
        ];

        for (i, piece) in back_rank.iter().enumerate() {
            let i = (i + 1) as i8;
            board.set_piece(Coordinates::new(i, 1), piece.clone());
            board.set_piece(
                Coordinates::new(i, 2),
                ChessPieceEnum::Pawn(Pawn::new(Color::White, Coordinates::new(i, 2))),
            );
            board.set_piece(
                Coordinates::new(i, 7),
                ChessPieceEnum::Pawn(Pawn::new(Color::Black, Coordinates::new(i, 7))),
            );

            // Update color and coordinates for the black pieces on the back rank
            let mut updated_piece = piece.clone();
            updated_piece.change_color_and_coordinates(Color::Black, Coordinates::new(i, 8));
            board.set_piece(Coordinates::new(i, 8), updated_piece);
        }

        board
    }

    /// Sets a piece at the given coordinates.
    pub fn set_piece(&mut self, coordinates: Coordinates, piece: ChessPieceEnum) {
        match &mut self.board_type {
            BoardType::OneDimensional(board) => {
                let index = coordinates.to_index();
                board[index] = Some(piece);
            }

            BoardType::TwoDimensional(board) => {
                let x = (coordinates.x - 1) as usize;
                let y = (coordinates.y - 1) as usize;
                board[y][x] = Some(piece);
            }
        }
    }

    /// Gets the piece at the given coordinates.
    pub fn get_piece(&self, coordinates: Coordinates) -> Option<&ChessPieceEnum> {
        match &self.board_type {
            BoardType::OneDimensional(board) => {
                let index = coordinates.to_index();
                board[index].as_ref()
            }
            BoardType::TwoDimensional(board) => {
                let x = (coordinates.x - 1) as usize;
                let y = (coordinates.y - 1) as usize;
                board[y][x].as_ref()
            }
        }
    }
}
