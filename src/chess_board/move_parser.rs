use crate::chess_board::coordinates::Coordinates;
use crate::chess_board::notation::Notation;
use crate::chess_piece::piece::{ChessPiece, Color, PieceType};

#[derive(Debug)]
pub struct ChessMove {
    source_piece: (PieceType, Color),
    source_location: Coordinates,
    captured_piece: Option<(PieceType, Color)>,
    captured_piece_location: Option<Coordinates>,
}

impl ChessMove {
    pub fn new(
        source_piece: (PieceType, Color),
        source_location: Coordinates,
        captured_piece: Option<(PieceType, Color)>,
        captured_piece_location: Option<Coordinates>,
    ) -> ChessMove {
        ChessMove {
            source_piece,
            source_location,
            captured_piece,
            captured_piece_location,
        }
    }
}

pub fn get_piece_from_char(piece_char: char) -> (PieceType, Color) {
    match piece_char {
        'p' => (PieceType::Pawn, Color::Black),
        'P' => (PieceType::Pawn, Color::White),
        'n' => (PieceType::Knight, Color::Black),
        'N' => (PieceType::Knight, Color::White),
        'b' => (PieceType::Bishop, Color::Black),
        'B' => (PieceType::Bishop, Color::White),
        'r' => (PieceType::Rook, Color::Black),
        'R' => (PieceType::Rook, Color::White),
        'q' => (PieceType::Queen, Color::Black),
        'Q' => (PieceType::Queen, Color::White),
        'k' => (PieceType::King, Color::Black),
        'K' => (PieceType::King, Color::White),
        _ => panic!("Invalid piece character: {}", piece_char),
    }
}
