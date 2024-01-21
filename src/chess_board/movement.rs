use crate::coordinates::Coordinates;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    from: Coordinates,
    to: Coordinates,
}

impl Move {
    pub fn new(from: Coordinates, to: Coordinates) -> Self {
        Self { from, to }
    }
}
