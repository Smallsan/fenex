# Fenex Chess Library

Fenex is a Rust library for handling chess game logic. It provides support for parsing Forsyth-Edwards Notation (FEN) strings, handling chess moves in algebraic notation, and more.

## Features

## Fen Module

The Fen module provides the `Fen` struct for representing a chess position in Forsyth-Edwards Notation (FEN).

### `Fen` Struct

- **Constructor:** `Fen::new(board: &str) -> Fen`
  - Creates a new `Fen` struct from a given board position string.

- **Parser:** `Fen::parse_fen(fen_str: &str) -> Result<Fen, &'static str>`
  - Parses a FEN string into a `Fen` struct.

- **Debug Printing:** `Fen::debug()`
  - Prints the values inside the `Fen` struct for debugging purposes.

- **Board Printing:** `Fen::print_to_board()`
  - Prints the chess board to the console.

- **Board Conversion:** `Fen::boardify() -> String`
  - Converts the `Fen` struct into a string representation of a chess board.

## Coordinates Module

The Coordinates module provides the `Coordinates` struct for representing a position on the chess board.

### `Coordinates` Struct

- **Constructor:** `Coordinates { x: i8, y: i8 }`
  - Creates a new `Coordinates` struct with given x and y values.

- **From Notation:** `Coordinates::from_notation(notation: Notation) -> Result<Coordinates, &'static str>`
  - Converts a `Notation` struct into a `Coordinates` struct.

- **To Notation:** `Coordinates::to_notation(&self) -> Result<Notation, &'static str>`
  - Converts the `Coordinates` struct into a `Notation` struct.

## Notation Module

The Notation module provides the `Notation` struct for representing a chess move in algebraic notation.

### `Notation` Struct

- **Constructor:** `Notation::new(file: char, rank: char) -> Option<Notation>`
  - Creates a new `Notation` struct with given file and rank values.

- **From Coordinates:** `Notation::from_coordinates(coordinates: Coordinates) -> Result<Notation, &'static str>`
  - Converts a `Coordinates` struct into a `Notation` struct.

- **To Coordinates:** `Notation::to_coordinates(&self) -> Result<Coordinates, &'static str>`
  - Converts the `Notation` struct into a `Coordinates` struct.

## ChessPieceEnum Module

The `ChessPieceEnum` module defines the `ChessPieceEnum` enum, representing different types of chess pieces, and provides methods for updating both color and coordinates.

## BoardType Module

The `BoardType` module defines the `BoardType` enum, representing different types of chess boards (1D and 2D).

## Board Module

The `Board` module provides the `Board` struct for handling chess boards in both 1D and 2D representations.

### `Board` Struct

- **Constructor:** `Board::new_one_dimensional() -> Board` and `Board::new_two_dimensional() -> Board`
  - Creates a new `1D` or `2D` board with all squares empty.

- **Starting Position:** `Board::new_one_dimensional_starting_position() -> Board` and `Board::new_two_dimensional_starting_position() -> Board`
  - Creates a new `1D` or `2D` board with pieces in the starting position.

- **Set Piece:** `Board::set_piece(&mut self, coordinates: Coordinates, piece: ChessPieceEnum)`
  - Sets a piece at the given coordinates.

- **Get Piece:** `Board::get_piece(&self, coordinates: Coordinates) -> Option<&ChessPieceEnum>`
  - Gets the piece at the given coordinates.

## How To Install

Just run the ```cargo add fenex```in your project directory.

Or add ```fenex = "0.1.2"``` in your Cargo.toml, Under dependencies.

## Sample Uses

### Coordinates and Notations

```rust
    fn create_notation_and_coordinates() {

        // Creates a Notation from chars. ('file' 'rank').
        let notation = Notation::new('e', '4').unwrap();

        // Creates Coordinates from an i8. (x, y).
        let coordinates = Coordinates::new(5, 4);

        // Creates a Notation from string. ("e4").
        let notation_from_string = Notation::from_string("e4").unwrap();

        // Creates Coordinates from a string of 2 i8 separated by a comma.
        // ("4.3").
        let coordinate_from_string = Coordinates::from_string("5,4").unwrap();
    }
```

### Boards

```rust
    fn move_piece_with_board() {

        // Creates a 1D board, With starting pieces.
        let mut one_dimensional_board = Board::new_one_dimensional_starting_position();

        // Creates Coordinates from a string of 2 i8 separated by a comma.
        let from = Coordinates::from_notation_string("e2").unwrap();

        // Creates Coordinates from a string of 2 i8 separated by a comma.
        let to = Coordinates::from_notation_string("e4").unwrap();

        // Displays the board.
        one_dimensional_board.print_board_with_labels();

        // Result of the move function.
        let res = one_dimensional_board.move_piece_with_coordinates(from, to);

        // Displays the updated board.
        one_dimensional_board.print_board_with_labels();
    }
```

## Contribution

Contributions to Fenex are welcome! Please ensure your code is formatted with `cargo fmt` before creating a pull request.
