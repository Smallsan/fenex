#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

pub trait ChessPiece {
    fn piece_type(&self) -> PieceType;
    fn color(&self) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct Pawn {
    pub color: Color,
}

impl ChessPiece for Pawn {
    fn piece_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn color(&self) -> Color {
        self.color
    }
}

pub enum Piece {
    Empty,
    Occupied(Box<dyn ChessPiece>),
}
