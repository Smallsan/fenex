use crate::chess::piece::piece::Color;

use super::board::Board;

pub enum GameState {
    Normal,
    Check,
    Checkmate,
    Stalemate,
}

impl Board {
    fn game_state(&self, color: Color) -> GameState {
        let king_position = self.find_king(color);
        let opponent_color = color.reverse();
        let opponent_moves = self.generate_moves(opponent_color);
        let king_moves = self.generate_moves(color);
        let king_moves_count = king_moves.len();
        let opponent_moves_count = opponent_moves.len();
        let is_king_in_check = self.is_king_in_check(color);

        if is_king_in_check && king_moves_count == 0 {
            return GameState::Checkmate;
        }

        if !is_king_in_check && opponent_moves_count == 0 {
            return GameState::Stalemate;
        }

        if is_king_in_check {
            return GameState::Check;
        }

        GameState::Normal
    }
}