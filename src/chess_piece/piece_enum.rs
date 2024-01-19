use crate::{
    bishop::Bishop,
    coordinates::Coordinates,
    king::King,
    knight::Knight,
    pawn::Pawn,
    piece::{ChessPiece, Color, PieceType},
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
    /// Returns chess piece type.
    pub fn get_piece_type(&self) -> PieceType {
        match self {
            ChessPieceEnum::Pawn(_) => PieceType::Pawn,
            ChessPieceEnum::Knight(_) => PieceType::Knight,
            ChessPieceEnum::Bishop(_) => PieceType::Bishop,
            ChessPieceEnum::Rook(_) => PieceType::Rook,
            ChessPieceEnum::Queen(_) => PieceType::Queen,
            ChessPieceEnum::King(_) => PieceType::King,
        }
    }

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

    /// Get a mutable reference to the underlying chess piece.
    pub fn get_piece_as_mut(&mut self) -> &mut dyn ChessPiece {
        match self {
            ChessPieceEnum::Pawn(pawn) => pawn,
            ChessPieceEnum::Knight(knight) => knight,
            ChessPieceEnum::Bishop(bishop) => bishop,
            ChessPieceEnum::Rook(rook) => rook,
            ChessPieceEnum::Queen(queen) => queen,
            ChessPieceEnum::King(king) => king,
        }
    }

    /// Sets the coordinates of the chess piece.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        match self {
            ChessPieceEnum::Pawn(pawn) => pawn.coordinates = coordinates,
            ChessPieceEnum::Rook(rook) => rook.coordinates = coordinates,
            ChessPieceEnum::Knight(knight) => knight.coordinates = coordinates,
            ChessPieceEnum::Bishop(bishop) => bishop.coordinates = coordinates,
            ChessPieceEnum::Queen(queen) => queen.coordinates = coordinates,
            ChessPieceEnum::King(king) => king.coordinates = coordinates,
        }
    }

    /// Convert the chess piece to a string representation.
    pub fn to_string(&self) -> String {
        match self {
            ChessPieceEnum::Pawn(pawn) => {
                if pawn.color == Color::White {
                    "P".to_string()
                } else {
                    "p".to_string()
                }
            }
            ChessPieceEnum::Knight(knight) => {
                if knight.color == Color::White {
                    "N".to_string()
                } else {
                    "n".to_string()
                }
            }
            ChessPieceEnum::Bishop(bishop) => {
                if bishop.color == Color::White {
                    "B".to_string()
                } else {
                    "b".to_string()
                }
            }
            ChessPieceEnum::Rook(rook) => {
                if rook.color == Color::White {
                    "R".to_string()
                } else {
                    "r".to_string()
                }
            }
            ChessPieceEnum::Queen(queen) => {
                if queen.color == Color::White {
                    "Q".to_string()
                } else {
                    "q".to_string()
                }
            }
            ChessPieceEnum::King(king) => {
                if king.color == Color::White {
                    "K".to_string()
                } else {
                    "k".to_string()
                }
            }
        }
    }
}
