#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

pub mod chess_board;
pub mod chess_piece;

pub use chess_board::{coordinates, fen_notation, notation};
pub use chess_piece::piece;
pub use chess_piece::pieces::{bishop, king, knight, pawn, queen, rook};

#[cfg(test)]
mod test {
    use crate::{chess_board::board::Board, coordinates::Coordinates, notation::Notation};

    #[test]
    // Creating a Notation and Coordinates object.
    fn create_notation_and_coordinates() {
        // Creates a Notation from chars. ('file' 'rank').
        let notation: Notation = Notation::new('e', '4').unwrap();

        // Creates Coordinates from an i8. (x, y).
        let coordinates: Coordinates = Coordinates::new(5, 4);

        // Creates a Notation from string. ("e4").
        let notation_from_string: Notation = Notation::from_string("e4").unwrap();

        // Creates Coordinates from a string of 2 i8 separated by a comma.
        // ("4.3").
        let coordinate_from_string: Coordinates = Coordinates::from_string("5,4").unwrap();
    }

    // Creating a board object
    #[test]
    fn create_chess_board() {
        // Creates a 2D board, With starting pieces.
        let two_dimensional_board = Board::new_two_dimensional_starting_position();

        // Creates a 1D board, With starting pieces.
        let one_dimensional_board = Board::new_one_dimensional_starting_position();
    }

    #[test]
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
}
