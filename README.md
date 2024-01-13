# Fenix Chess Library (Unfinished)

Fenix is a Rust library for handling chess game logic. It provides support for parsing Forsyth-Edwards Notation (FEN) strings, handling chess moves in algebraic notation, and more.

## Features

### Fen Module

The Fen module provides the `Fen` struct for representing a chess position in Forsyth-Edwards Notation (FEN).

#### `Fen` Struct

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

## Contribution

Contributions to Fenix are welcome! Please ensure your code is formatted with `cargo fmt` before creating a pull request.
