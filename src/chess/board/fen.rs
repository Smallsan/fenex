use crate::chess::piece::{
    piece::{CastlingRights, Color},
    piece_enum::ChessPieceEnum,
    types::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook},
};

use super::{
    board::{Board, BoardType},
    board_enum::BoardTypeEnum,
    coordinates::Coordinates,
};

/// Represents a FEN (Forsyth–Edwards Notation) chess position.

#[derive(Debug)]

pub struct Fen {
    board: String,
    turn: Option<String>,
    castling: Option<CastlingRights>,
    en_passant: Option<Coordinates>,
    halfmove_clock: Option<u32>,
    fullmove_number: Option<u32>,
}

impl Fen {
    /// Parses a FEN string and returns a `Fen` instance representing the position.
    ///
    /// # Arguments
    ///
    /// * `fen_str` - The FEN string to parse.
    ///
    /// # Returns
    ///
    /// * `Result<Fen, &'static str>` - A `Result` containing the parsed `Fen` instance or an error message.
    pub fn new(fen_str: &str) -> Result<Fen, &'static str> {
        let parts: Vec<&str> = fen_str.trim().split_whitespace().collect();

        if parts.len() < 2 {
            return Err("Invalid FEN string");
        }

        let mut fen = Fen {
            board: String::from(parts[0]),
            turn: None,
            castling: None,
            en_passant: None,
            halfmove_clock: None,
            fullmove_number: None,
        };

        for (i, &part) in parts.iter().skip(1).enumerate() {
            match i {
                0 => fen.turn = Some(String::from(part)),
                1 => fen.castling = Some(CastlingRights::from_str(&String::from(part))),
                2 => fen.en_passant = Coordinates::from_string(&String::from(part)),
                3 => fen.halfmove_clock = Some(part.parse().unwrap_or_default()),
                4 => fen.fullmove_number = Some(part.parse().unwrap_or_default()),
                _ => return Err("Invalid FEN string"),
            }
        }

        Ok(fen)
    }
    /// Prints the debug information of the `Fen` instance.
    pub fn debug(&self) {
        println!(
            "Turn: {}\nCastling: {:?}\nEn Passant: {:?}\nHalfmove Clock: {}\nFullmove Number: {}",
            self.turn.as_deref().unwrap_or("-"),
            self.castling.unwrap_or_default(),
            self.en_passant.map_or("-".to_string(), |coords| coords.to_string()),
            self.halfmove_clock.unwrap_or_default(),
            self.fullmove_number.unwrap_or_default(),
        );
    }

    /// Prints the board representation of the `Fen` instance.
    pub fn display(&self) {
        Fen::to_board(&self, BoardTypeEnum::OneDimensional).display();
    }

    pub fn to_board(&self, board_type: BoardTypeEnum) -> Board {
        let mut board = match board_type {
            BoardTypeEnum::OneDimensional => Board::new_one_dimensional(),
            BoardTypeEnum::TwoDimensional => Board::new_two_dimensional(),
        };

        let rows: Vec<&str> = self.board.split('/').collect();
        for (i, row) in rows.iter().rev().enumerate() {
            let mut j = 0;
            for square in row.chars() {
                match square {
                    '1'..='8' => {
                        let num_empty: usize = square.to_digit(10).unwrap() as usize;
                        j += num_empty;
                    }
                    piece => {
                        let color = if piece.is_uppercase() {
                            Color::White
                        } else {
                            Color::Black
                        };
                        let piece_type = match piece.to_ascii_lowercase() {
                            'p' => ChessPieceEnum::Pawn(Pawn::new(
                                color,
                                Coordinates::from_index(j + i * 8),
                                false,
                            )),
                            'n' => ChessPieceEnum::Knight(Knight::new(
                                color,
                                Coordinates::from_index(j + i * 8),
                            )),
                            'b' => ChessPieceEnum::Bishop(Bishop::new(
                                color,
                                Coordinates::from_index(j + i * 8),
                            )),
                            'r' => ChessPieceEnum::Rook(Rook::new(
                                color,
                                Coordinates::from_index(j + i * 8),
                            )),
                            'q' => ChessPieceEnum::Queen(Queen::new(
                                color,
                                Coordinates::from_index(j + i * 8),
                            )),
                            'k' => ChessPieceEnum::King(King::new(
                                color,
                                Coordinates::from_index(j + i * 8),
                            )),
                            _ => unreachable!(),
                        };
                        match board.board_type {
                            BoardType::OneDimensional(ref mut b) => b[j + i * 8] = Some(piece_type),
                            BoardType::TwoDimensional(ref mut b) => b[i][j] = Some(piece_type),
                        }
                        j += 1;
                    }
                }
            }
        }
        board.color_to_move = match self.turn.as_deref() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            _ => panic!("Invalid turn in FEN"),
        };
        board.is_in_check = false; // This information is not available in FEN
        board.castling_rights = self.castling;
        board.en_passant_target = self.en_passant;
        board.halfmove_clock = Some(self.halfmove_clock.unwrap_or(0));
        board.fullmove_number = Some(self.fullmove_number.unwrap_or(1));
    
        // Check if the king is in check
        if board.is_king_in_check(board.color_to_move) {
            board.is_in_check = true;
        }
    
        board
    }
}
