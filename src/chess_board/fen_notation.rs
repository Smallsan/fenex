/// Represents a FEN (Forsyth–Edwards Notation) chess position.

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
    /// Creates a new `Fen` instance with the given board position.
    ///
    /// # Arguments
    ///
    /// * `board` - The FEN string representing the board position.
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

    /// Parses a FEN string and returns a `Fen` instance representing the position.
    ///
    /// # Arguments
    ///
    /// * `fen_str` - The FEN string to parse.
    ///
    /// # Returns
    ///
    /// * `Result<Fen, &'static str>` - A `Result` containing the parsed `Fen` instance or an error message.
    pub fn parse_fen(fen_str: &str) -> Result<Fen, &'static str> {
        let parts: Vec<&str> = fen_str.trim().split_whitespace().collect();

        if parts.len() < 2 {
            return Err("Invalid FEN string");
        }

        let mut fen = Fen::new(parts[0]);

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

    /// Prints the debug information of the `Fen` instance.
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

    /// Prints the board representation of the `Fen` instance.
    pub fn print_to_board(&self) {
        println!("{}", self.boardify())
    }

    /// Converts the board position to a string representation.
    ///
    /// # Returns
    ///
    /// * `String` - The string representation of the board position.
    pub fn boardify(&self) -> String {
        let mut result = String::new();

        let rows: Vec<&str> = self.board.split('/').collect();

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
