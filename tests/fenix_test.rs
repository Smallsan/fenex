extern crate fenix;

use fenix::chess_board::fen::*;

#[test]
fn main() {
    let fen = Fen::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Failed to parse FEN");
    fen.print_to_board();
}
