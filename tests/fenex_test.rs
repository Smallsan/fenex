extern crate fenex;

use fenex::bishop_piece::*;
use fenex::chess_board::fen_notation::*;
use fenex::chess_board::move_parser::*;

use fenex::coordinates::Coordinates;
use fenex::notation::Notation;
use fenex::piece::ChessPiece;
use fenex::piece::Color;
use fenex::rook_piece::Rook;

#[test]
fn main() {
    let fen = Fen::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Failed to parse FEN");
    fen.print_to_board();
    let notation = Notation::new('e', '4').unwrap();
    dbg!(notation);
    let coords = Coordinates::from_notation(notation).unwrap();
    dbg!(coords);

    let mut bishop1 = Bishop::new(Color::Black, coords);
    let bishop2 = Bishop::new(Color::White, Coordinates::new(6, 5));
    dbg!(bishop1);
}
