extern crate fenix;

use fenix::chess_board::fen::*;
use fenix::chess_board::algebraic_notation::*;

#[test]
fn main() {
    let fen = Fen::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Failed to parse FEN");
    fen.print_to_board();


    println!("{:?}", coord_first);
    println!("{:?}", coord_second);

}