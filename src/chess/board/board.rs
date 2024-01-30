use crate::chess::piece::{
    piece::{CastlingRights, Color},
    piece_enum::ChessPieceEnum,
};

use super::{board_enum::BoardTypeEnum, coordinates::Coordinates, fen::Fen, notation::Notation};

/// The BoardType represents the different types of chess boards.
#[derive(Clone, Debug, PartialEq)]
pub enum BoardType {
    OneDimensional(Vec<Option<ChessPieceEnum>>),
    TwoDimensional([[Option<ChessPieceEnum>; 8]; 8]),
}

/// The Board represents a chess board.
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub board_type: BoardType,
    pub color_to_move: Color,
    pub is_in_check: bool,
    pub castling_rights: Option<CastlingRights>,
    pub en_passant_target: Option<Coordinates>,
    pub halfmove_clock: Option<u32>,
    pub fullmove_number: Option<u32>,
}

impl Board {
    /// Creates a new `1D Board` with all squares empty.
    pub fn new_one_dimensional() -> Self {
        Board {
            board_type: BoardType::OneDimensional(vec![None; 64]),
            color_to_move: Color::White,
            is_in_check: false,
            castling_rights: Some(CastlingRights::default()),
            en_passant_target: None,
            halfmove_clock: Some(0),
            fullmove_number: Some(1),
        }
    }

    pub fn clone(&self) -> Self {
        Board {
            board_type: self.board_type.clone(),
            color_to_move: self.color_to_move,
            is_in_check: self.is_in_check,
            castling_rights: self.castling_rights,
            en_passant_target: self.en_passant_target,
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.fullmove_number,
        }
    }

    /// Creates a new `2D Board` with all squares empty.
    pub fn new_two_dimensional() -> Self {
        Board {
            board_type: BoardType::TwoDimensional([[None; 8]; 8]),
            color_to_move: Color::White,
            is_in_check: false,
            castling_rights: Some(CastlingRights::default()),
            en_passant_target: None,
            halfmove_clock: Some(0),
            fullmove_number: Some(1),
        }
    }

    /// Creates a new `1D Board` with pieces in the starting position.
    pub fn new_one_dimensional_starting_position() -> Self {
        let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let board = fen.to_board(BoardTypeEnum::OneDimensional);

        board
    }

    /// Creates a new `2D Board` with pieces in the starting position.
    pub fn new_two_dimensional_starting_position() -> Self {
        let fen = Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let board = fen.to_board(BoardTypeEnum::TwoDimensional);

        board
    }

    /// Sets a piece at the given coordinates.
    pub fn set_piece(&mut self, coordinates: Coordinates, piece: ChessPieceEnum) {
        match &mut self.board_type {
            BoardType::OneDimensional(board) => {
                let index = coordinates.to_index();
                board[index.expect("Index is out of bounds")] = Some(piece);
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
            BoardType::OneDimensional(board) => match coordinates.to_index() {
                Ok(index) if index < board.len() => board[index].as_ref(),
                _ => None,
            },
            BoardType::TwoDimensional(board) => {
                let x = (coordinates.x - 1) as usize;
                let y = (coordinates.y - 1) as usize;
                board[y][x].as_ref()
            }
        }
    }

    /// Moves a piece from one set of coordinates to another.
    pub fn move_piece_with_coordinates(
        &mut self,
        from: Coordinates,
        to: Coordinates,
    ) -> Result<(), &'static str> {
        // Get the piece at the 'from' coordinates.
        let mut piece_enum = match self.get_piece(from) {
            Some(piece) => piece.clone(),
            None => return Err("No piece at the source coordinates"),
        };

        let piece = piece_enum.get_piece_as_mut();

        // Check if the move is valid.
        if !piece.is_valid_move(to, self, true) {
            println!("invalid move");
            return Err("Invalid move");
        }

        // Remove the piece from the 'from' coordinates.
        self.remove_piece(from);

        // Update the piece's coordinates.
        piece.change_coordinates(to);

        // Place the piece at the 'to' coordinates.
        self.set_piece(to, piece_enum);

        Ok(())
    }

    /// Moves a piece from one set of Notations to another.
    pub fn move_piece_with_notation(
        &mut self,
        from: Notation,
        to: Notation,
    ) -> Result<(), &'static str> {
        let from_coordinates = from.to_coordinates().unwrap();
        let to_coordinates = to.to_coordinates().unwrap();
        self.move_piece_with_coordinates(from_coordinates, to_coordinates)
    }

    /// Returns the color to move.
    pub fn get_color_to_move(&self) -> &Color {
        &self.color_to_move
    }

    /// Changes the color to move.
    pub fn change_color_to_move(&mut self) {
        self.color_to_move = match self.color_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    /// Returns the board type.
    pub fn get_board_type(&self) -> &BoardType {
        &self.board_type
    }

    /// Force moves a piece from the source coordinates to the destination coordinates.
    pub fn move_piece(&mut self, source: Coordinates, destination: Coordinates) {
        // Remove the piece from the source coordinates.
        let piece = self.remove_piece(source);

        // Place the piece at the destination coordinates.
        if let Some(mut piece) = piece {
            // Update the piece's coordinates to the destination.
            piece.set_coordinates(destination);
            self.set_piece(destination, piece);
        }
    }

    /// Force removes a piece from the given coordinates and returns it.
    pub fn remove_piece(&mut self, coordinates: Coordinates) -> Option<ChessPieceEnum> {
        match &mut self.board_type {
            BoardType::OneDimensional(board) => {
                let index = coordinates.to_index();
                board[index.expect("Index is out of bounds")].take()
            }
            BoardType::TwoDimensional(board) => {
                let x = (coordinates.x - 1) as usize;
                let y = (coordinates.y - 1) as usize;
                board[y][x].take()
            }
        }
    }

    /// Print the board with notations.
    pub fn display(&self) {
        match &self.board_type {
            BoardType::OneDimensional(board) => {
                for i in (0..8).rev() {
                    // Print rank label
                    print!("{} ", i + 1);

                    for j in 0..8 {
                        if let Some(piece) = board[i * 8 + j] {
                            print!("[{}]", piece.to_string());
                        } else {
                            print!("[ ]");
                        }
                    }
                    println!();
                }
            }
            BoardType::TwoDimensional(board) => {
                for (i, row) in board.iter().enumerate().rev() {
                    // Print rank label
                    print!("{} ", i + 1);

                    for piece in row {
                        if let Some(piece) = piece {
                            print!("[{}]", piece.to_string());
                        } else {
                            print!("[ ]");
                        }
                    }
                    println!();
                }
            }
        }
        // Print file labels
        println!("   a  b  c  d  e  f  g  h");
    }
}
