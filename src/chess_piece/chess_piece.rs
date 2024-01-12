#[derive(Debug, Clone, Copy)]
/// Possible piece types in chess.
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
/// Possible colors of chess pieces.
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl ChessPiece {
    pub fn new(piece_type: PieceType, color: Color) -> ChessPiece {
        ChessPiece { piece_type, color }
    }

}