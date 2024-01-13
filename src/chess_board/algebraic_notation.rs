use crate::chess_piece::chess_piece::{Color, PieceType, ChessPiece, Coordinates};

#[derive(Debug)]
pub struct ChessMove {
    source_piece: (PieceType, Color),
    source_location: Coordinates,
    captured_piece: Option<(PieceType, Color)>,
    captured_piece_location: Option<Coordinates>,
}

pub struct Notation {
    file: char,
    rank: char,
}


impl ChessMove {
    pub fn new(
        source_piece: (PieceType, Color),
        source_location: Coordinates,
        captured_piece: Option<(PieceType, Color)>,
        captured_piece_location: Option<Coordinates>,
    ) -> ChessMove {
        ChessMove {
            source_piece: source_piece,
            source_location: source_location,
            captured_piece: captured_piece,
            captured_piece_location: captured_piece_location,
        }
    }
}

pub fn parse_notation(input: &str) -> Result<ChessMove, &str> {
    let chars: Vec<char> = input.chars().collect();

    let source_piece = get_piece_from_char(chars[0]);
    let source_location = Notation{file: chars[1], rank: chars[2]};
    let _capture_symbol = chars[3];
    let captured_piece = get_piece_from_char(chars[4]);
    let captured_piece_location = Notation{file: chars[5], rank: chars[6]};

    Ok(ChessMove::new(
        source_piece,
        convert_to_coordinates(source_location),
        Some(captured_piece),
        Some(convert_to_coordinates(captured_piece_location)),
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

pub fn convert_to_coordinates(notation:Notation) -> Coordinates {
    let mut x = 0;
    let mut y = 0;
    match notation.file {
        'a' => x = 0,
        'b' => x = 1,
        'c' => x = 2,
        'd' => x = 3,
        'e' => x = 4,
        'f' => x = 5,
        'g' => x = 6,
        'h' => x = 7,
        _ => panic!("Invalid character: {}", notation.file),
    }
    match notation.rank {
        '1' => y = 0,
        '2' => y = 1,
        '3' => y = 2,
        '4' => y = 3,
        '5' => y = 4,
        '6' => y = 5,
        '7' => y = 6,
        '8' => y = 7,
        _ => panic!("Invalid character: {}", notation.rank),
    }
    Coordinates{x,y}
}

pub fn convert_to_notation(coordinates: Coordinates) -> Notation {
    let mut file = 'a';
    let mut rank = '1';
    match coordinates.x {
        0 => file = 'a',
        1 => file = 'b',
        2 => file = 'c',
        3 => file = 'd',
        4 => file = 'e',
        5 => file = 'f',
        6 => file = 'g',
        7 => file = 'h',
        _ => panic!("Invalid character: {}", coordinates.x),
    }
    match coordinates.y {
        0 => rank = '1',
        1 => rank = '2',
        2 => rank = '3',
        3 => rank = '4',
        4 => rank = '5',
        5 => rank = '6',
        6 => rank = '7',
        7 => rank = '8',
        _ => panic!("Invalid character: {}", coordinates.y),
    }
    Notation{file,rank}
}