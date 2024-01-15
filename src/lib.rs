#![doc = include_str!("../README.md")]

pub mod chess_board;
pub mod chess_piece;

pub use chess_board::{coordinates, fen_notation, move_parser, notation};
pub use chess_piece::piece;
pub use chess_piece::pieces::{
    bishop_piece, king_piece, knight_piece, pawn_piece, queen_piece, rook_piece,
};

#[cfg(test)]
mod test {
    use crate::{chess_board::board::Board, coordinates::*, notation::*};

    /// The notation and coordinates struct
    #[test]
    fn create_notation_and_coordinates() {
        let notation = Notation::new('e', '4').unwrap();
        let coordinates = Coordinates::new(5, 4);

        assert_eq!(coordinates, notation.to_coordinates().unwrap());
        assert_eq!(notation, coordinates.to_notation().unwrap());

        let notation_from_char = Notation::from_char('e', '4').unwrap();
        let notation_from_string = Notation::from_string("e4").unwrap();
        assert_eq!(notation_from_char, notation_from_string);

        let coordinate_from_new = Coordinates::new(4, 3);
        let coordinate_from_string = Coordinates::from_string("4,3").unwrap();
        assert_eq!(coordinate_from_string, coordinate_from_new);
    }

    /// Creating a board object
    #[test]
    fn create_chess_board() {
        let two_dimensional_board = Board::new_two_dimensional_starting_position();
        dbg!(two_dimensional_board);
        let one_dimensional_board = Board::new_one_dimensional_starting_position();
        dbg!(one_dimensional_board);
    }
}
