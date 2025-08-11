#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

pub mod chess;

#[cfg(test)]
mod test {
    use crate::chess::board::board::Board;

    /// Check and fix pawns?
    /// Check if the new move gen is working.
    /// Convert fen into boards.
    /// Create something to visualize moves.
    /// When pawns are moved they can no longer be moved?

    //#[test]
    // fn notation_and_coordinates() {
    //     // Example usage for Notation and Coordinates APIs
    //     let notation = Notation::new('e', '4').unwrap();
    //     let coordinates = Coordinates::new(5, 4);
    //     let notation_from_string = Notation::from_string("e4").unwrap();
    //     let coordinate_from_string = Coordinates::from_string("4,3").unwrap();
    // }

    #[test]
    fn boards_and_moves() {
        let fen = "r1b4r/ppkppppq/4P2p/7n/RNK2b1n/Q7/PPPPP1PP/2B2BNR w - - 0 1";
        let board = Board::from_fen(fen).unwrap();
        board.display();
        assert_eq!(board.to_fen(), fen);
    }
}
