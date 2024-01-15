#![doc = include_str!("../README.md")]

pub mod chess_board;
pub mod chess_piece;

pub use chess_board::{coordinates, fen_notation, move_parser, notation};
pub use chess_piece::piece;
pub use chess_piece::pieces::{bishop, king, knight, pawn, queen, rook};

#[cfg(test)]
mod test {
    use crate::{chess_board::board::Board, coordinates::Coordinates, notation::Notation};

    #[test]
    // Creating a Notation and Coordinates object.
    fn create_notation_and_coordinates() {
        // Creates a Notation from chars. ('file' 'rank').
        let notation = Notation::new('e', '4').unwrap();

        // Creates Coordinates from an i8. (x, y).
        let coordinates = Coordinates::new(5, 4);

        // Checks if the converted notation is equal to coordinates.
        assert_eq!(coordinates, notation.to_coordinates().unwrap());
        // Checks if the converted coordinates is equal to notation.
        assert_eq!(notation, coordinates.to_notation().unwrap());

        // Creates a Notation from string. ("e4").
        let notation_from_string = Notation::from_string("e4").unwrap();

        // Creates Coordinates from a string of 2 i8 separated by a comma.
        // ("4.3").
        let coordinate_from_string = Coordinates::from_string("5,4").unwrap();

        // Checks if the converted notation is equal to coordinates.
        assert_eq!(
            coordinate_from_string,
            notation_from_string.to_coordinates().unwrap()
        );

        // Checks if the converted coordinates is equal to notation.
        assert_eq!(
            notation_from_string,
            coordinate_from_string.to_notation().unwrap()
        );
    }

    // Creating a board object
    #[test]
    fn create_chess_board() {
        // Creates a 2D board, With starting pieces.
        let two_dimensional_board = Board::new_two_dimensional_starting_position();

        // Creates a 1D board, With starting pieces.
        let one_dimensional_board = Board::new_one_dimensional_starting_position();

        // For checking the boards.
        dbg!(one_dimensional_board);
        dbg!(two_dimensional_board);
    }
}
