use crate::{coordinates::Coordinates, piece::PieceType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    piece_type: PieceType,
    from: Coordinates,
    to: Coordinates,
}

impl Move {
    pub fn new(from: Coordinates, to: Coordinates, piece_type: PieceType) -> Self {
        Self {
            from,
            to,
            piece_type,
        }
    }

    pub fn from(&self) -> Coordinates {
        self.from
    }

    pub fn to(&self) -> Coordinates {
        self.to
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}
