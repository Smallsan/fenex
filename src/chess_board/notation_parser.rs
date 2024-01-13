use crate::chess_piece::chess_piece::{Color, PieceType, ChessPiece};
use crate::chess_board::coordinates::Coordinates;
use crate::chess_board::notation::Notation;

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

pub fn parse_notation(input: &str) -> Result<ChessMove, &str> {
    let chars: Vec<char> = input.chars().collect();

    Ok(ChessMove::new(
        get_piece_from_char(chars[0]),
        convert_to_coordinates(Notation{file: chars[1], rank: chars[2]}),
        Some(get_piece_from_char(chars[4])),
        Some(convert_to_coordinates(Notation{file: chars[5], rank: chars[6]})),
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

pub fn convert_to_coordinates(notation: Notation) -> Coordinates {
    let x = match notation.file {
        'a'..='h' => (notation.file as u8 - 'a' as u8) as usize,
        _ => panic!("Invalid character: {}", notation.file),
    };
    let y = match notation.rank {
        '1'..='8' => (notation.rank as u8 - '1' as u8) as usize,
        _ => panic!("Invalid character: {}", notation.rank),
    };
    Coordinates { x: x as i8, y: y as i8 }
}

pub fn convert_to_notation(coordinates: Coordinates) -> Notation {
    let file = (coordinates.x as u8 + 'a' as u8) as char;
    let rank = (coordinates.y as u8 + '1' as u8) as char;
    Notation { file, rank }
}