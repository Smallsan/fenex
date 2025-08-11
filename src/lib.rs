#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

pub mod chess;

#[cfg(test)]
mod test {
    use crate::chess::board::board::Board;

    #[test]
    fn boards_and_moves() {
        let fen = "r1b4r/ppkppppq/4P2p/7n/RNK2b1n/Q7/PPPPP1PP/2B2BNR w - - 0 1";
        let board = Board::from_fen(fen).unwrap();
        board.display();
        assert_eq!(board.to_fen(), fen);
    }
}
