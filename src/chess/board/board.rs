impl Board {
    /// Executes a move with full rule validation
    pub fn apply_move(&mut self, from: Coordinates, to: Coordinates) -> Result<(), &'static str> {
        // Validate move legality first
        let legal_moves = self.generate_legal_moves();
        if !legal_moves.contains(&(from, to)) {
            return Err("Illegal move");
        }
        
        // Handle castling
        if let Some(piece) = self.get(from) {
            if piece.piece_type == PieceType::King {
                let rank = from.y;
                // Kingside castling
                if from.x == 5 && to.x == 7 {
                    self.set(Coordinates { x: 8, y: rank }, None);
                    self.set(Coordinates { x: 6, y: rank }, Some(Piece { piece_type: PieceType::Rook, color: piece.color, has_moved: true }));
                }
                // Queenside castling
                if from.x == 5 && to.x == 3 {
                    self.set(Coordinates { x: 1, y: rank }, None);
                    self.set(Coordinates { x: 4, y: rank }, Some(Piece { piece_type: PieceType::Rook, color: piece.color, has_moved: true }));
                }
                // Update castling rights
                match piece.color {
                    Color::White => {
                        self.castling_rights[0] = false;
                        self.castling_rights[1] = false;
                    }
                    Color::Black => {
                        self.castling_rights[2] = false;
                        self.castling_rights[3] = false;
                    }
                }
            }
            // Update castling rights when rook moves
            if piece.piece_type == PieceType::Rook {
                match (piece.color, from.x, from.y) {
                    (Color::White, 1, 1) => self.castling_rights[1] = false, // White queenside
                    (Color::White, 8, 1) => self.castling_rights[0] = false, // White kingside
                    (Color::Black, 1, 8) => self.castling_rights[3] = false, // Black queenside
                    (Color::Black, 8, 8) => self.castling_rights[2] = false, // Black kingside
                    _ => {}
                }
            }
        }
        let piece = self.get(from);
        // Handle en passant capture
        if let Some(p) = piece {
            if p.piece_type == PieceType::Pawn {
                if let Some(ep) = self.en_passant {
                    if to == ep && (from.x - to.x).abs() == 1 && (from.y - to.y).abs() == 1 && self.get(to).is_none() {
                        // Remove captured pawn
                        let cap_y = if p.color == Color::White { to.y - 1 } else { to.y + 1 };
                        self.set(Coordinates { x: to.x, y: cap_y }, None);
                    }
                }
            }
        }
        // Execute the move
        self.set(from, None);
        self.set(to, piece);
        
        // Handle pawn promotion
        if let Some(mut p) = piece {
            if p.piece_type == PieceType::Pawn {
                let promotion_rank = if p.color == Color::White { 8 } else { 1 };
                if to.y == promotion_rank {
                    p.piece_type = PieceType::Queen;
                    self.set(to, Some(p));
                }
            }
        }
        
        // Update en passant square
        if let Some(p) = piece {
            if p.piece_type == PieceType::Pawn && (from.y - to.y).abs() == 2 {
                let ep_y = (from.y + to.y) / 2;
                self.en_passant = Some(Coordinates { x: from.x, y: ep_y });
            } else {
                self.en_passant = None;
            }
        } else {
            self.en_passant = None;
        }
        self.color_to_move = match self.color_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        Ok(())
    }

    /// Locates the king for the given color
    pub fn find_king(&self, color: Color) -> Option<Coordinates> {
        for y in 1..=8 {
            for x in 1..=8 {
                let coord = Coordinates { x, y };
                if let Some(piece) = self.get(coord) {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return Some(coord);
                    }
                }
            }
        }
        None
    }
    /// Generate all legal moves for the current player.
    pub fn generate_legal_moves(&self) -> Vec<(Coordinates, Coordinates)> {
        let mut legal_moves = Vec::new();
        for y in 1..=8 {
            for x in 1..=8 {
                let from = Coordinates { x, y };
                if let Some(piece) = self.get(from) {
                    if piece.color == self.color_to_move {
                        let pseudo_moves = self.generate_piece_moves(from, piece);
                        for to in pseudo_moves {
                            let mut clone = self.clone();
                            // Directly apply the move without legality check to avoid recursion
                            let moving_piece = clone.get(from);
                            clone.set(from, None);
                            clone.set(to, moving_piece);
                            clone.color_to_move = match clone.color_to_move {
                                Color::White => Color::Black,
                                Color::Black => Color::White,
                            };
                            if !clone.is_in_check() {
                                legal_moves.push((from, to));
                            }
                        }
                    }
                }
            }
        }
        legal_moves
    }

    /// Check if the current player is in check.
    pub fn is_in_check(&self) -> bool {
    let king_pos = match self.find_king(self.color_to_move) {
            Some(pos) => pos,
            None => return false,
        };
        let opponent = match self.color_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.squares[y][x] {
                    if piece.color == opponent {
                        let from = Coordinates { x: (x + 1) as i8, y: (y + 1) as i8 };
                        let moves = self.generate_piece_moves(from, piece);
                        if moves.contains(&king_pos) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Check if the current player is in checkmate.
    pub fn is_checkmate(&self) -> bool {
        self.is_in_check() && self.generate_legal_moves().is_empty()
    }

    /// Check if the current player is in stalemate.
    pub fn is_stalemate(&self) -> bool {
        !self.is_in_check() && self.generate_legal_moves().is_empty()
    }

    /// Generate all moves for a piece at a given position (ignores check).
    fn generate_piece_moves(&self, from: Coordinates, piece: Piece) -> Vec<Coordinates> {
        let mut moves = Vec::new();
        match piece.piece_type {
            PieceType::Pawn => {
                let dir = if piece.color == Color::White { 1 } else { -1 };
                let next_y = from.y + dir;
                if next_y >= 1 && next_y <= 8 {
                    // Forward
                    let forward = Coordinates { x: from.x, y: next_y };
                    if self.get(forward).is_none() {
                        moves.push(forward);
                        // Double move
                        let start_row = if piece.color == Color::White { 2 } else { 7 };
                        if from.y == start_row {
                            let double_forward = Coordinates { x: from.x, y: from.y + 2 * dir };
                            if self.get(double_forward).is_none() {
                                moves.push(double_forward);
                            }
                        }
                    }
                    // Captures
                    for dx in [-1, 1].iter() {
                        let nx = from.x + dx;
                        if nx >= 1 && nx <= 8 {
                            let capture = Coordinates { x: nx, y: next_y };
                            if let Some(target) = self.get(capture) {
                                if target.color != piece.color {
                                    moves.push(capture);
                                }
                            } else if let Some(ep) = self.en_passant {
                                if capture == ep {
                                    moves.push(capture);
                                }
                            }
                        }
                    }
                }
            }
            PieceType::Knight => {
                let knight_moves = [
                    (2, 1), (1, 2), (-1, 2), (-2, 1),
                    (-2, -1), (-1, -2), (1, -2), (2, -1),
                ];
                for (dx, dy) in knight_moves.iter() {
                    let nx = from.x + dx;
                    let ny = from.y + dy;
                    if nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
                        let to = Coordinates { x: nx, y: ny };
                        if let Some(target) = self.get(to) {
                            if target.color != piece.color {
                                moves.push(to);
                            }
                        } else {
                            moves.push(to);
                        }
                    }
                }
            }
            PieceType::Bishop => {
                let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                for (dx, dy) in directions.iter() {
                    let mut nx = from.x + dx;
                    let mut ny = from.y + dy;
                    while nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
                        let to = Coordinates { x: nx, y: ny };
                        if let Some(target) = self.get(to) {
                            if target.color != piece.color {
                                moves.push(to);
                            }
                            break;
                        } else {
                            moves.push(to);
                        }
                        nx += dx;
                        ny += dy;
                    }
                }
            }
            PieceType::Rook => {
                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for (dx, dy) in directions.iter() {
                    let mut nx = from.x + dx;
                    let mut ny = from.y + dy;
                    while nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
                        let to = Coordinates { x: nx, y: ny };
                        if let Some(target) = self.get(to) {
                            if target.color != piece.color {
                                moves.push(to);
                            }
                            break;
                        } else {
                            moves.push(to);
                        }
                        nx += dx;
                        ny += dy;
                    }
                }
            }
            PieceType::Queen => {
                let directions = [
                    (1, 1), (1, -1), (-1, 1), (-1, -1),
                    (1, 0), (-1, 0), (0, 1), (0, -1),
                ];
                for (dx, dy) in directions.iter() {
                    let mut nx = from.x + dx;
                    let mut ny = from.y + dy;
                    while nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
                        let to = Coordinates { x: nx, y: ny };
                        if let Some(target) = self.get(to) {
                            if target.color != piece.color {
                                moves.push(to);
                            }
                            break;
                        } else {
                            moves.push(to);
                        }
                        nx += dx;
                        ny += dy;
                    }
                }
            }
            PieceType::King => {
                let king_moves = [
                    (1, 0), (1, 1), (0, 1), (-1, 1),
                    (-1, 0), (-1, -1), (0, -1), (1, -1),
                ];
                for (dx, dy) in king_moves.iter() {
                    let nx = from.x + dx;
                    let ny = from.y + dy;
                    if nx >= 1 && nx <= 8 && ny >= 1 && ny <= 8 {
                        let to = Coordinates { x: nx, y: ny };
                        if let Some(target) = self.get(to) {
                            if target.color != piece.color {
                                moves.push(to);
                            }
                        } else {
                            moves.push(to);
                        }
                    }
                }
                // Castling
                let (rank, kingside, queenside) = match piece.color {
                    Color::White => (1, self.castling_rights[0], self.castling_rights[1]),
                    Color::Black => (8, self.castling_rights[2], self.castling_rights[3]),
                };
                // Helper to check if a square is attacked
                let is_attacked = |sq: Coordinates| {
                    let mut clone = self.clone();
                    clone.color_to_move = match piece.color {
                        Color::White => Color::Black,
                        Color::Black => Color::White,
                    };
                    for y in 1..=8 {
                        for x in 1..=8 {
                            let from = Coordinates { x, y };
                            if let Some(attacker) = clone.get(from) {
                                if attacker.color == clone.color_to_move {
                                    if attacker.piece_type == PieceType::King {
                                        // King attacks adjacent squares only
                                        if (from.x - sq.x).abs() <= 1 && (from.y - sq.y).abs() <= 1 {
                                            return true;
                                        }
                                    } else {
                                        let attacks = clone.generate_piece_moves(from, attacker);
                                        if attacks.contains(&sq) {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    false
                };
                // Kingside
                if kingside
                    && self.get(Coordinates { x: 6, y: rank }).is_none()
                    && self.get(Coordinates { x: 7, y: rank }).is_none()
                    && self.get(Coordinates { x: 8, y: rank }).map_or(false, |r| r.piece_type == PieceType::Rook && r.color == piece.color)
                    && !is_attacked(Coordinates { x: 5, y: rank })
                    && !is_attacked(Coordinates { x: 6, y: rank })
                    && !is_attacked(Coordinates { x: 7, y: rank })
                {
                    moves.push(Coordinates { x: 7, y: rank });
                }
                // Queenside
                if queenside
                    && self.get(Coordinates { x: 4, y: rank }).is_none()
                    && self.get(Coordinates { x: 3, y: rank }).is_none()
                    && self.get(Coordinates { x: 2, y: rank }).is_none()
                    && self.get(Coordinates { x: 1, y: rank }).map_or(false, |r| r.piece_type == PieceType::Rook && r.color == piece.color)
                    && !is_attacked(Coordinates { x: 5, y: rank })
                    && !is_attacked(Coordinates { x: 4, y: rank })
                    && !is_attacked(Coordinates { x: 3, y: rank })
                {
                    moves.push(Coordinates { x: 3, y: rank });
                }
            }
        }
        moves
    }
}

use crate::chess::board::coordinates::Coordinates;
use crate::chess::piece::piece::{Color, PieceType};

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub has_moved: bool,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => 'P',
            (PieceType::Knight, Color::White) => 'N',
            (PieceType::Bishop, Color::White) => 'B',
            (PieceType::Rook, Color::White) => 'R',
            (PieceType::Queen, Color::White) => 'Q',
            (PieceType::King, Color::White) => 'K',
            (PieceType::Pawn, Color::Black) => 'p',
            (PieceType::Knight, Color::Black) => 'n',
            (PieceType::Bishop, Color::Black) => 'b',
            (PieceType::Rook, Color::Black) => 'r',
            (PieceType::Queen, Color::Black) => 'q',
            (PieceType::King, Color::Black) => 'k',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub color_to_move: Color,
    pub castling_rights: [bool; 4], // [w_kingside, w_queenside, b_kingside, b_queenside]
    pub en_passant: Option<Coordinates>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [[None; 8]; 8],
            color_to_move: Color::White,
            castling_rights: [true, true, true, true],
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut board = Board::new();
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("Invalid FEN: not enough parts");
        }
        // Piece placement
        let mut rank = 8;
        let mut file = 1;
        for c in parts[0].chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 1;
                }
                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as i8;
                }
                'p' | 'n' | 'b' | 'r' | 'q' | 'k' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                    let (piece_type, color) = match c {
                        'p' => (PieceType::Pawn, Color::Black),
                        'n' => (PieceType::Knight, Color::Black),
                        'b' => (PieceType::Bishop, Color::Black),
                        'r' => (PieceType::Rook, Color::Black),
                        'q' => (PieceType::Queen, Color::Black),
                        'k' => (PieceType::King, Color::Black),
                        'P' => (PieceType::Pawn, Color::White),
                        'N' => (PieceType::Knight, Color::White),
                        'B' => (PieceType::Bishop, Color::White),
                        'R' => (PieceType::Rook, Color::White),
                        'Q' => (PieceType::Queen, Color::White),
                        'K' => (PieceType::King, Color::White),
                        _ => unreachable!(),
                    };
                    let piece = Piece {
                        piece_type,
                        color,
                        has_moved: false,
                    };
                    board.set(Coordinates { x: file, y: rank }, Some(piece));
                    file += 1;
                }
                _ => return Err("Invalid FEN: invalid character in piece placement"),
            }
        }
    // The rest of the FEN parsing logic (active color, castling rights, en passant, etc.) follows here, outside the piece placement loop.
        // Active color
        board.color_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid FEN: invalid active color"),
        };
        // Castling rights
        board.castling_rights = [false; 4];
        if parts[2].contains('K') {
            board.castling_rights[0] = true;
        }
        if parts[2].contains('Q') {
            board.castling_rights[1] = true;
        }
        if parts[2].contains('k') {
            board.castling_rights[2] = true;
        }
        if parts[2].contains('q') {
            board.castling_rights[3] = true;
        }
        // En passant
        if parts[3] != "-" {
            let file = (parts[3].as_bytes()[0] - b'a' + 1) as i8;
            let rank = (parts[3].as_bytes()[1] - b'1' + 1) as i8;
            board.en_passant = Some(Coordinates { x: file, y: rank });
        }
        // Halfmove clock
        if parts.len() > 4 {
            board.halfmove_clock = parts[4].parse().unwrap_or(0);
        }
        // Fullmove number
        if parts.len() > 5 {
            board.fullmove_number = parts[5].parse().unwrap_or(1);
        }
        Ok(board)
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (1..=8).rev() {
            let mut empty = 0;
            for file in 1..=8 {
                match self.get(Coordinates { x: file, y: rank }) {
                    Some(piece) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        let c = match (piece.piece_type, piece.color) {
                            (PieceType::Pawn, Color::White) => 'P',
                            (PieceType::Knight, Color::White) => 'N',
                            (PieceType::Bishop, Color::White) => 'B',
                            (PieceType::Rook, Color::White) => 'R',
                            (PieceType::Queen, Color::White) => 'Q',
                            (PieceType::King, Color::White) => 'K',
                            (PieceType::Pawn, Color::Black) => 'p',
                            (PieceType::Knight, Color::Black) => 'n',
                            (PieceType::Bishop, Color::Black) => 'b',
                            (PieceType::Rook, Color::Black) => 'r',
                            (PieceType::Queen, Color::Black) => 'q',
                            (PieceType::King, Color::Black) => 'k',
                        };
                        fen.push(c);
                    }
                    None => empty += 1,
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            if rank > 1 {
                fen.push('/');
            }
        }
        // Active color
        fen.push(' ');
        fen.push(match self.color_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });
        // Castling rights
        fen.push(' ');
        let mut rights = String::new();
        if self.castling_rights[0] {
            rights.push('K');
        }
        if self.castling_rights[1] {
            rights.push('Q');
        }
        if self.castling_rights[2] {
            rights.push('k');
        }
        if self.castling_rights[3] {
            rights.push('q');
        }
        if rights.is_empty() {
            rights.push('-');
        }
        fen.push_str(&rights);
        // En passant
        fen.push(' ');
        if let Some(ep) = self.en_passant {
            let file = (b'a' + (ep.x - 1) as u8) as char;
            let rank = (b'1' + (ep.y - 1) as u8) as char;
            fen.push(file);
            fen.push(rank);
        } else {
            fen.push('-');
        }
        // Halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());
        // Fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());
        fen
    }

    pub fn get(&self, coord: Coordinates) -> Option<Piece> {
        let (x, y) = ((coord.x - 1) as usize, (coord.y - 1) as usize);
        self.squares[y][x]
    }

    pub fn set(&mut self, coord: Coordinates, piece: Option<Piece>) {
        let (x, y) = ((coord.x - 1) as usize, (coord.y - 1) as usize);
        self.squares[y][x] = piece;
    }

    pub fn display(&self) {
        for y in (0..8).rev() {
            print!("{} ", y + 1);
            for x in 0..8 {
                match self.squares[y][x] {
                    Some(piece) => print!("[{}]", piece),
                    None => print!("[ ]"),
                }
            }
            println!();
        }
        println!("   a  b  c  d  e  f  g  h");
    }
}
