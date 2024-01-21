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

Or add `fenex = "0.1.3"` in your Cargo.toml, Under dependencies.

## Examples

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
