# Fenex Chess Library

Fenex is a Rust library for handling chess game logic. It provides support for parsing Forsyth-Edwards Notation (FEN) strings, handling chess moves in algebraic notation, and more.

## Features

## Coordinates And Notations

Both Coordinates and Notations are 1-indexed.

### `Coordinates` Struct

The Coordinates module provides the `Coordinates` struct for representing a position on the chess board.

- **Constructor:** `Coordinates { x: i8, y: i8 }`

  - Creates a new `Coordinates` struct with given x and y values.

- **From Notation:** `Coordinates::from_notation(notation: Notation) -> Result<Coordinates, &'static str>`

  - Converts a `Notation` struct into a `Coordinates` struct.

- **To Notation:** `Coordinates::to_notation(&self) -> Result<Notation, &'static str>`
  - Converts the `Coordinates` struct into a `Notation` struct.

### `Notation` Struct

The Notation module provides the `Notation` struct for representing a chess move in algebraic notation.

- **Constructor:** `Notation::new(file: char, rank: char) -> Option<Notation>`

  - Creates a new `Notation` struct with given file and rank values.

- **From Coordinates:** `Notation::from_coordinates(coordinates: Coordinates) -> Result<Notation, &'static str>`

  - Converts a `Coordinates` struct into a `Notation` struct.

- **To Coordinates:** `Notation::to_coordinates(&self) -> Result<Coordinates, &'static str>`
  - Converts the `Notation` struct into a `Coordinates` struct.

## Boards And Movements

### `Board` Struct

The `Board` module provides the `Board` struct for handling chess boards in both 1D and 2D representations.

- **Constructor:** `Board::new_one_dimensional() -> Board` and `Board::new_two_dimensional() -> Board`

  - Creates a new `1D` or `2D` board with all squares empty.

- **Starting Position:** `Board::new_one_dimensional_starting_position() -> Board` and `Board::new_two_dimensional_starting_position() -> Board`

  - Creates a new `1D` or `2D` board with pieces in the starting position.

- **Set Piece:** `Board::set_piece(&mut self, coordinates: Coordinates, piece: ChessPieceEnum)`

  - Sets a piece at the given coordinates.

- **Get Piece:** `Board::get_piece(&self, coordinates: Coordinates) -> Option<&ChessPieceEnum>`

  - Gets the piece at the given coordinates.

- **Generate Moves:** `Board::generate_moves(&self, color: Color) -> Vec<Move>`
  - Generates all valid moves for the pieces of the given color on the board.

### `Move` Struct

The `Move` struct represents a move in a chess game. It contains the type of the piece being moved and the start and end coordinates of the move.

- **Constructor:** `Move::new(from: Coordinates, to: Coordinates, piece_type: PieceType) -> Move`
  - Creates a new `Move` with the given start and end coordinates, and the type of the piece being moved.

## How To Install

Just run the `cargo add fenex`in your project directory.

Or add `fenex = "0.1.4"` in your Cargo.toml, Under dependencies.

## Examples

```rust
fn notation_and_coordinates() {
    // Create a Notation from chars ('file' 'rank').
    let notation: Notation = Notation::new('e', '4').unwrap();

    // Create Coordinates from an i8 (x, y).
    let coordinates: Coordinates = Coordinates::new(5, 4);

    // Create a Notation from a string ("e4").
    let notation_from_string: Notation = Notation::from_string("e4").unwrap();

    // Create Coordinates from a string of 2 i8 separated by a comma ("5,4").
    let coordinate_from_string: Coordinates = Coordinates::from_string("5,4").unwrap();
}

fn boards_and_moves() {
    // Create a 2D board with starting pieces.
    let mut two_dimensional_board = Board::new_two_dimensional_starting_position();

    // Create a 1D board with starting pieces.
    let mut one_dimensional_board = Board::new_one_dimensional_starting_position();

    // Create Coordinates from a string of 2 i8 separated by a comma ("e2").
    let from = Coordinates::from_notation_string("e2").unwrap();

    // Create Coordinates from a string of 2 i8 separated by a comma ("e4").
    let to = Coordinates::from_notation_string("e4").unwrap();

    // Move a piece from one coordinate to another.
    one_dimensional_board.move_piece_with_coordinates(from, to);

    // Generate all possible movements for White, The boolean check_for_checks filters out illegal moves that would leave the king in check.
    let movement = one_dimensional_board.generate_moves(Color::White, true);
}

fn fen() {
    // Create a fen object.
    let fen = Fen::new("r1b4r/ppkppppq/4P2p/7n/RNK2b1n/Q7/PPPPP1PP/2B2BNR w - - 0 1").unwrap();

    // Parse the fen into a one-dimensional board.
    let one_dimensional_board = fen.to_board(BoardTypeEnum::OneDimensional);

    // Parse the fen into a two-dimensional board.
    let two_dimensional_board = fen.to_board(BoardTypeEnum::TwoDimensional);
}
```

## Contribution

Contributions to Fenex are welcome! Please ensure your code is formatted with `cargo fmt` before creating a pull request.
