use crate::chess_board::{board::Board, coordinates::Coordinates};

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
    fn coordinates(&self) -> Coordinates;
    fn change_color(&mut self, color: Color);
    fn change_coordinates(&mut self, coordinates: Coordinates);
    fn move_to(&self, destination: Coordinates, board: &mut Board) -> Result<(), &'static str>;
    fn is_valid_move(&self, coordinates: Coordinates, board: &Board) -> bool;
}
