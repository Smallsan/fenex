use crate::chess::{board::board::Board, piece::piece::Color};

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

        let moves = self.generate_moves(color);
        let opponent_moves = self.generate_moves(opponent_color);

        let moves_count = moves.len();
        let opponent_moves_count = opponent_moves.len();

        let is_king_in_check = self.is_king_in_check(color);

        if is_king_in_check && moves_count == 0 {
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

    // Checks if the king is undercheck.
    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_position = self.find_king(color);
        let opponent_color = color.reverse();
        let opponent_moves = self.generate_moves(opponent_color);
        opponent_moves
            .iter()
            .any(|m| m.to() == king_position.expect("Couldn't find the king's position"))
    }
}
