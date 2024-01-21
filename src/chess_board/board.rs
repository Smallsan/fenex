use crate::chess_board::movement::Move;
use crate::{
    bishop::Bishop, chess_piece::piece_enum::ChessPieceEnum, coordinates::Coordinates, king::King,
    knight::Knight, notation::Notation, pawn::Pawn, piece::Color, queen::Queen, rook::Rook,
};

/// The BoardType represents the different types of chess boards.
#[derive(Debug, PartialEq)]
pub enum BoardType {
    OneDimensional(Vec<Option<ChessPieceEnum>>),
    TwoDimensional([[Option<ChessPieceEnum>; 8]; 8]),
}

/// The Board represents a chess board.
#[derive(Debug, PartialEq)]
pub struct Board {
    pub board_type: BoardType,
    pub color_to_move: Color,
    pub is_in_check: bool,
}

impl Board {
    /// Creates a new `1D Board` with all squares empty.
    pub fn new_one_dimensional() -> Self {
        Board {
            board_type: BoardType::OneDimensional(vec![None; 64]),
            color_to_move: Color::White,
            is_in_check: false,
        }
    }
    /// Creates a new `2D Board` with all squares empty.
    pub fn new_two_dimensional() -> Self {
        Board {
            board_type: BoardType::TwoDimensional([[None; 8]; 8]),
            color_to_move: Color::White,
            is_in_check: false,
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
                has_moved: false,
            }));
            board[i + 40] = Some(ChessPieceEnum::Pawn(Pawn {
                color: Color::Black,
                coordinates: Coordinates::from_index(i + 40),
                has_moved: false,
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
            color_to_move: Color::White,
            is_in_check: false,
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
                ChessPieceEnum::Pawn(Pawn::new(Color::White, Coordinates::new(i, 2), false)),
            );
            board.set_piece(
                Coordinates::new(i, 7),
                ChessPieceEnum::Pawn(Pawn::new(Color::Black, Coordinates::new(i, 7), false)),
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
        if !piece.is_valid_move(to, self) {
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
    pub fn get_color_to_move(&self) -> Color {
        self.color_to_move
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

    /// Print the board.
    pub fn print_board(&self) {
        match &self.board_type {
            BoardType::OneDimensional(board) => {
                for (i, piece) in board.iter().enumerate() {
                    if let Some(piece) = piece {
                        print!("[{}]", piece.to_string());
                    } else {
                        print!("[ ]");
                    }
                    if (i + 1) % 8 == 0 {
                        println!();
                    }
                }
            }
            BoardType::TwoDimensional(board) => {
                for row in board {
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

    /// Force moves a piece from the given coordinates and returns it.
    fn remove_piece(&mut self, coordinates: Coordinates) -> Option<ChessPieceEnum> {
        match &mut self.board_type {
            BoardType::OneDimensional(board) => {
                let index = coordinates.to_index();
                board[index].take()
            }
            BoardType::TwoDimensional(board) => {
                let x = (coordinates.x - 1) as usize;
                let y = (coordinates.y - 1) as usize;
                board[y][x].take()
            }
        }
    }

    /// Print the board with notations.
    pub fn print_board_with_labels(&self) {
        // Print file labels
        println!("   a  b  c  d  e  f  g  h");

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
    }
    pub fn generate_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        match &self.board_type {
            BoardType::OneDimensional(board) => {
                for i in 1..=64 {
                    if let Some(piece) = board[i - 1] {
                        if piece.color() == color {
                            for x_to in 1..=8 {
                                for y_to in 1..=8 {
                                    let destination = Coordinates::new(x_to, y_to);
                                    let piece_type = piece.piece_type();
                                    if piece.is_valid_move(destination, self) {
                                        moves.push(Move::new(
                                            piece.coordinates(),
                                            destination,
                                            piece_type,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            BoardType::TwoDimensional(board) => {
                for x_from in 1..=8 {
                    for y_from in 1..=8 {
                        if let Some(piece) = board[x_from - 1][y_from - 1] {
                            if piece.color() == color {
                                for x_to in 1..=8 {
                                    for y_to in 1..=8 {
                                        let piece_type = piece.piece_type();
                                        let destination = Coordinates::new(x_to, y_to);
                                        if piece.is_valid_move(destination, self) {
                                            moves.push(Move::new(
                                                piece.coordinates(),
                                                destination,
                                                piece_type,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }
}
