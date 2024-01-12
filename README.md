# Fenix Chess Library (Incomplete)
## Features

### Fen Module

#### `Fen` Struct

- **Constructor:** `Fen::new(board: &str) -> Fen`
  - Creates a new Fen struct.

- **Parser:** `Fen::parse_fen(fen_str: &str) -> Result<Fen, &'static str>`
  - Parses a FEN string into a Fen struct.

- **Debug Printing:** `Fen::debug()`
  - Prints the values inside the Fen struct.

- **Board Printing:** `Fen::print_to_board()`
  - Prints the chess board using the `boardify` function.

- **Board Conversion:** `Fen::boardify() -> String`
  - Returns a chess board as a String.

### Algebraic Notation Module

#### `ChessMove` Struct

- **Functions:**
  - `get_piece_from_char(piece_char: char) -> (PieceType, Color)`
    - Converts a character to a `PieceType` and `Color`.

  - `parse_attack_notation()`
    - Parses algebraic attack notations.

## Contribution

- Prior to creating a pull request, ensure code formatting with `cargo fmt`.