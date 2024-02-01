#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

pub mod chess;

#[cfg(test)]
mod test {
    use crate::chess::{
        board::{
            board::{Board, BoardType},
            board_enum::BoardTypeEnum,
            coordinates::Coordinates,
            fen::Fen,
            notation::Notation,
        },
        piece::piece::Color,
    };

    /// Check and fix pawns?
    /// Check if the new move gen is working.
    /// Convert fen into boards.
    /// Create something to visualize moves.
    /// When pawns are moved they can no longer be moved?

    //#[test]
    fn notation_and_coordinates() {
        // Creates a Notation from chars. ('file' 'rank').
        let notation: Notation = Notation::new('e', '4').unwrap();

        // Creates Coordinates from an i8. (x, y).
        let coordinates: Coordinates = Coordinates::new(5, 4);

        // Creates a Notation from string. ("e4").
        let notation_from_string: Notation = Notation::from_string("e4").unwrap();

        // Creates Coordinates from a string of 2 i8 separated by a comma.
        // ("4.3").
        let coordinate_from_string: Coordinates = Coordinates::from_string("4,3").unwrap();
    }

    #[test]
    fn boards_and_moves() {
        // Creates a 2D board, With starting pieces.
        let mut two_dimensional_board = Board::new_two_dimensional_starting_position();

        // Creates a 1D board, With starting pieces.
        let mut one_dimensional_board = Board::new_one_dimensional_starting_position();

        let fen = Fen::new(
            "r1b4r/ppkppppq/4P2p/7n/RNK2b1n/Q7/PPPPP1PP/2B2BNR w - - 0 1
        ",
        )
        .unwrap();

        let board = fen.to_board(BoardTypeEnum::OneDimensional);

        board.display();

        fen.display();
    }
}
