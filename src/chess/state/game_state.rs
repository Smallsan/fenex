use crate::chess::{
    board::{
        board::{Board, BoardType},
        coordinates::Coordinates,
    },
    piece::piece::{Color, PieceType},
};

pub enum GameState {
    Normal,
    Check,
    Checkmate,
    Stalemate,
}

impl Board {
    /// Returns the game state.
    pub fn game_state(&self, color: Color) -> GameState {
        let opponent_color = color.reverse();

        let moves = self.generate_moves(color, true);
        let opponent_moves = self.generate_moves(opponent_color, true);

        let moves_count = moves.len();
        let opponent_moves_count = opponent_moves.len();

        let is_king_in_check = self.is_king_in_check(color);

        // If the king is in check and has no moves, it's checkmate.
        if is_king_in_check && moves_count == 0 {
            return GameState::Checkmate;
        }

        // If the king is not in check and has no moves, it's stalemate.
        if !is_king_in_check && opponent_moves_count == 0 {
            return GameState::Stalemate;
        }

        // If the king is in check and has moves, it's check.
        if is_king_in_check {
            return GameState::Check;
        }

        GameState::Normal
    }

    // Checks if the king is undercheck.
    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_position = match self.find_king(color) {
            Some(position) => position,
            None => return false,
        };

        let opponent_color = color.reverse();
        let opponent_moves = self.generate_moves(opponent_color, false);
        opponent_moves.iter().any(|m| m.to() == king_position)
    }

    /// finds the king piece in the board and returns it's coordinates.
    pub fn find_king(&self, color: Color) -> Option<Coordinates> {
        match &self.board_type {
            BoardType::OneDimensional(board) => {
                for i in 1..=64 {
                    if let Some(piece) = board[i - 1] {
                        if piece.color() == color && piece.piece_type() == PieceType::King {
                            return Some(piece.coordinates());
                        }
                    }
                }
            }
            BoardType::TwoDimensional(board) => {
                for x in 1..=8 {
                    for y in 1..=8 {
                        if let Some(piece) = board[x - 1][y - 1] {
                            if piece.color() == color && piece.piece_type() == PieceType::King {
                                return Some(piece.coordinates());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
