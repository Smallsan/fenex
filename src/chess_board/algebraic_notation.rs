use crate::chess_piece::chess_piece::{Color, PieceType};

#[derive(Debug)]
pub struct ChessMove {
    source_piece: (PieceType, Color),
    source_location: (char, char),
    captured_piece: (PieceType, Color),
    captured_piece_location: (char, char),
}

impl ChessMove {
    pub fn new(
        source_piece: (PieceType, Color),
        source_location: (char, char),
        captured_piece: (PieceType, Color),
        captured_piece_location: (char, char),
    ) -> ChessMove {
        ChessMove {
            source_piece: source_piece,
            source_location: source_location,
            captured_piece: captured_piece,
            captured_piece_location: captured_piece_location,
        }
    }
}

pub fn parse_attack_notation(input: &str) -> Result<ChessMove, &str> {
    let chars: Vec<char> = input.chars().collect();

    let source_piece = get_piece_from_char(chars[0]);
    let source_location = (chars[1], chars[2]);
    let _capture_symbol = chars[3];
    let captured_piece = get_piece_from_char(chars[4]);
    let captured_piece_location = (chars[5], chars[6]);

    Ok(ChessMove::new(
        source_piece,
        source_location,
        captured_piece,
        captured_piece_location,
    ))
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
