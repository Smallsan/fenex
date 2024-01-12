#[derive(Debug)]
pub struct Fen {
    board: String,
    turn: Option<String>,
    castling: Option<String>,
    en_passant: Option<String>,
    halfmove_clock: Option<u32>,
    fullmove_number: Option<u32>,
}

impl Fen {
    pub fn new(board: &str) -> Fen {
        Fen {
            board: String::from(board),
            turn: None,
            castling: None,
            en_passant: None,
            halfmove_clock: None,
            fullmove_number: None,
        }
    }

    /// Parses string into Fen struct.
    /// Ex. Fen String: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    pub fn parse_fen(fen_str: &str) -> Result<Fen, &'static str> {
        // Split FEN string into parts
        let parts: Vec<&str> = fen_str.trim().split_whitespace().collect();

        // Validate that there are at least two parts (board and turn)
        if parts.len() < 2 {
            return Err("Invalid FEN string");
        }

        // Create a new Fen instance with the board part
        let mut fen = Fen::new(parts[0]);

        // Iterate over the remaining parts and update the Fen struct
        for (i, &part) in parts.iter().skip(1).enumerate() {
            match i {
                0 => fen.turn = Some(String::from(part)),
                1 => fen.castling = Some(String::from(part)),
                2 => fen.en_passant = Some(String::from(part)),
                3 => fen.halfmove_clock = Some(part.parse().unwrap_or_default()),
                4 => fen.fullmove_number = Some(part.parse().unwrap_or_default()),
                _ => return Err("Invalid FEN string"),
            }
        }

        Ok(fen)
    }

    /// Prints out the values inside the Fen struct.
    pub fn debug(&self) {
        println!(
            "Turn: {}\nCastling: {}\nEn Passant: {}\nHalfmove Clock: {}\nFullmove Number: {}",
            self.turn.as_deref().unwrap_or("-"),
            self.castling.as_deref().unwrap_or("-"),
            self.en_passant.as_deref().unwrap_or("-"),
            self.halfmove_clock.unwrap_or_default(),
            self.fullmove_number.unwrap_or_default(),
        );
    }

    /// Prints out a chess board.
    /// It just uses the boardify function and prints its output
    pub fn print_to_board(&self) {
        println!("{}", self.boardify())
    }

    /// Returns a chess board in form of a String.
    /// It loops through each square in each row and matches it.
    pub fn boardify(&self) -> String {
        let mut result = String::new();

        // Split the board string into rows
        let rows: Vec<&str> = self.board.split('/').collect();

        // Print the chessboard
        for row in rows.iter().rev() {
            for square in row.chars() {
                match square {
                    '1'..='8' => {
                        let num_empty: usize = square.to_digit(10).unwrap() as usize;
                        result.push_str(&format!("{} ", "{ } ".repeat(num_empty)));
                    }
                    piece @ ('P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q'
                    | 'k') => {
                        result.push_str(&format!("{{{}}} ", piece));
                    }
                    _ => unreachable!(),
                }
            }
            result.push('\n');
        }
        return result;
    }
}
