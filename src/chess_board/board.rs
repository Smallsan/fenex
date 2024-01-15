use crate::{
    bishop_piece::Bishop, king_piece::King, knight_piece::Knight, pawn_piece::Pawn,
    queen_piece::Queen, rook_piece::Rook,
};

pub enum ChessPiece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

pub enum BoardType {
    OneDimensional(Vec<Option<ChessPiece>>),
    TwoDimensional([[Option<ChessPiece>; 8]; 8]),
}

pub struct Board {
    board_type: BoardType,
}
