# Fenex Chess Library

A fast and reliable chess engine in Rust with full rule implementation and FEN support.

## Features

- ✅ Complete chess rules: castling, en passant, pawn promotion
- ✅ Legal move generation with check validation  
- ✅ FEN parsing and serialization
- ✅ Coordinate system with algebraic notation support
- ✅ Comprehensive test suite

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
fenex = "0.1.6"
```

## Basic Usage

```rust
use fenex::chess::board::board::Board;
use fenex::chess::board::coordinates::Coordinates;

// Load position from FEN
let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

// Make a move (e2 to e4)  
board.apply_move(Coordinates::new(5, 2), Coordinates::new(5, 4)).unwrap();

// Generate all legal moves
let moves = board.generate_legal_moves();
println!("Legal moves: {}", moves.len());

// Export back to FEN
println!("Position: {}", board.to_fen());
```

## Architecture

### Coordinates
- 1-indexed system (a1 = Coordinates::new(1,1))
- Conversion to/from algebraic notation

### Board  
- 8x8 array representation
- FEN import/export
- Legal move validation
- Special moves (castling, en passant, promotion)

### Pieces
- All standard chess pieces with proper movement rules
- Color and piece type tracking

## Examples

### Basic Game Setup
```rust
use fenex::chess::board::board::Board;
use fenex::chess::board::coordinates::Coordinates;

// Start new game
let mut board = Board::new();

// Or load from FEN
let mut board = Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3").unwrap();
```

### Making Moves  
```rust
// Move piece from e2 to e4
let from = Coordinates::new(5, 2);
let to = Coordinates::new(5, 4);
board.apply_move(from, to).unwrap();

// Check if move is legal first
let legal_moves = board.generate_legal_moves();
if legal_moves.contains(&(from, to)) {
    board.apply_move(from, to).unwrap();
}
```

### Special Moves
```rust
// Castling (automatically moves rook)
board.apply_move(Coordinates::new(5, 1), Coordinates::new(7, 1)).unwrap();

// En passant capture  
board.apply_move(Coordinates::new(5, 5), Coordinates::new(4, 6)).unwrap();

// Pawn promotion (defaults to Queen)
board.apply_move(Coordinates::new(1, 7), Coordinates::new(1, 8)).unwrap();
```

## Testing

Run the test suite:
```bash
cargo test
```

All chess rules are thoroughly tested including edge cases for special moves.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality  
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request
