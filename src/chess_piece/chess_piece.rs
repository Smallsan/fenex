use crate::chess_board::coordinates::Coordinates;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub trait ChessPiece {
    fn piece_type(&self) -> PieceType;
    fn color(&self) -> Color;
    fn location(&self) -> Coordinates;
    fn move_piece(&self, from: Coordinates, to: Coordinates) -> Result<(), &'static str>;
    fn can_capture(&self, target: &dyn ChessPiece) -> bool;
    fn can_move_to(&self, location: Coordinates) -> bool;
}
