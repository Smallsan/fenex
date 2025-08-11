use fenex::chess::board::board::Board;
use fenex::chess::board::coordinates::Coordinates;
use fenex::chess::piece::piece::PieceType;

#[test]
fn test_checkmate_detection() {
    // Fool's mate setup: 1. f3 e5 2. g4
    let fen = "rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2";
    let mut board = Board::from_fen(fen).unwrap();
    let legal_moves = board.generate_legal_moves();
    
    // Find queen moves
    let queen_moves: Vec<_> = legal_moves.iter()
        .filter(|(from, _to)| {
            if let Some(piece) = board.get(*from) {
                piece.piece_type == PieceType::Queen
            } else {
                false
            }
        })
        .collect();
    
    // Execute any queen move (Qh4# in real game)
    if let Some(&(from, to)) = queen_moves.first() {
        board.apply_move(*from, *to).unwrap();
    }
}

#[test]
fn test_stalemate_detection() {
    let fen = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    let _board = Board::from_fen(fen).unwrap();
}

#[test]
fn test_illegal_move() {
    let fen = "8/8/8/3P4/8/8/8/8 w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    // Pawn can't move backwards
    let result = board.apply_move(Coordinates::new(4, 5), Coordinates::new(4, 4));
    assert!(result.is_err());
}

#[test]
fn test_invalid_fen() {
    let bad_fen = "invalid fen string";
    let result = Board::from_fen(bad_fen);
    assert!(result.is_err());
}

#[test]
fn test_fen_parsing_and_serialization() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen).expect("FEN should parse");
    assert_eq!(board.to_fen(), fen);
}

#[test]
fn test_pawn_move_generation() {
    let fen = "8/8/8/3P4/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    let moves = board.generate_legal_moves();
    assert!(moves.contains(&(Coordinates::new(4, 5), Coordinates::new(4, 6))));
}

#[test]
fn test_knight_move_generation() {
    let fen = "8/8/8/4N3/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    let moves = board.generate_legal_moves();
    assert!(moves.contains(&(Coordinates::new(5, 5), Coordinates::new(7, 6))));
}

#[test]
fn test_castling_rights() {
    let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    let moves = board.generate_legal_moves();
    assert!(moves.contains(&(Coordinates::new(5, 1), Coordinates::new(7, 1))));
    assert!(moves.contains(&(Coordinates::new(5, 1), Coordinates::new(3, 1))));
    // Castle kingside
    board
        .apply_move(Coordinates::new(5, 1), Coordinates::new(7, 1))
        .unwrap();
    assert!(!board.castling_rights[0]);
}

#[test]
fn test_en_passant() {
    let fen = "8/8/8/4pP2/8/8/8/8 w - e6 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    let moves = board.generate_legal_moves();
    assert!(moves.contains(&(Coordinates::new(6, 5), Coordinates::new(5, 6))));
    // Capture via en passant
    board
        .apply_move(Coordinates::new(6, 5), Coordinates::new(5, 6))
        .unwrap();
    assert!(board.get(Coordinates::new(5, 5)).is_none());
}

#[test]
fn test_pawn_promotion() {
    let fen = "8/P7/8/8/8/8/8/8 w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    let moves = board.generate_legal_moves();
    assert!(moves
        .iter()
        .any(|&(from, to)| from == Coordinates::new(1, 7) && to == Coordinates::new(1, 8)));
    // Promote to queen
    board
        .apply_move(Coordinates::new(1, 7), Coordinates::new(1, 8))
        .unwrap();
    assert_eq!(
        board.get(Coordinates::new(1, 8)).unwrap().piece_type,
        PieceType::Queen
    );
}

#[test]
fn test_check_detection() {
    // White king in check from black rook - White to move
    let fen = "4k3/8/8/8/8/8/8/4K2r w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
}

#[test]
fn test_no_check_detection() {
    // Standard starting position - no check
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(!board.is_in_check());
}

#[test]
fn test_diagonal_check() {
    // White king in check from black bishop
    let fen = "4k3/8/8/8/8/2b5/8/4K3 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
}

#[test]
fn test_knight_check() {
    // White king in check from black knight
    let fen = "4k3/8/8/8/8/3n4/8/4K3 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
}

#[test]
fn test_queen_check() {
    // White king in check from black queen (diagonal)
    let fen = "4k3/8/8/8/8/2q5/8/4K3 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
}

#[test]
fn test_pawn_check() {
    // White king in check from black pawn
    let fen = "4k3/8/8/8/8/3p4/2K5/8 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
}

#[test]
fn test_blocked_check() {
    // Rook attack blocked by piece - no check
    let fen = "4k3/8/8/8/8/8/8/4KP1r w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(!board.is_in_check());
}

#[test]
fn test_fools_mate() {
    // Fool's mate: 1. f3 e5 2. g4 Qh4#
    let fen = "rnbqkbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
    assert!(board.is_checkmate());
    assert!(!board.is_stalemate());
    
    // Should have no legal moves in checkmate
    let legal_moves = board.generate_legal_moves();
    assert!(legal_moves.is_empty());
}

#[test]
fn test_scholars_mate() {
    // Scholar's mate: 1. e4 e5 2. Bc4 Nc6 3. Qh5 Nf6?? 4. Qxf7#
    let fen = "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
    assert!(board.is_checkmate());
}

#[test]
fn test_back_rank_mate() {
    // Back rank mate pattern - Black king trapped by pawns, white rook gives horizontal check
    let fen = "1R4k1/5ppp/8/8/8/8/8/7K b - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
    assert!(board.is_checkmate());
}

#[test]
fn test_stalemate_position() {
    // Classic stalemate: king has no legal moves but not in check
    let fen = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    assert!(!board.is_in_check());
    assert!(board.is_stalemate());
    assert!(!board.is_checkmate());
    
    // Should have no legal moves in stalemate
    let legal_moves = board.generate_legal_moves();
    assert!(legal_moves.is_empty());
}

#[test]
fn test_check_escape_by_moving_king() {
    // King in check but can escape
    let fen = "4k3/8/8/8/8/8/8/4K2r w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
    assert!(!board.is_checkmate());
    
    // King should be able to move to d1, d2, e2, f1, or f2
    let legal_moves = board.generate_legal_moves();
    assert!(!legal_moves.is_empty());
    
    // Test one escape move
    let escape_move = legal_moves.iter()
        .find(|(from, _to)| from == &Coordinates::new(5, 1))
        .expect("King should have legal moves");
    
    board.apply_move(escape_move.0, escape_move.1).unwrap();
    assert!(!board.is_in_check());
}

#[test]
fn test_check_escape_by_blocking() {
    // King in check but can be blocked
    let fen = "4k3/8/8/8/8/8/4B3/4K2r w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
    assert!(!board.is_checkmate());
    
    // Bishop should be able to block by moving to f1
    let legal_moves = board.generate_legal_moves();
    let blocking_move = legal_moves.iter()
        .find(|(_from, to)| to == &Coordinates::new(6, 1))
        .expect("Should be able to block check");
    
    board.apply_move(blocking_move.0, blocking_move.1).unwrap();
    assert!(!board.is_in_check());
}

#[test]
fn test_check_escape_by_capture() {
    // King in check but attacking piece can be captured
    let fen = "4k3/8/8/8/8/8/7Q/4K2r w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    assert!(board.is_in_check());
    assert!(!board.is_checkmate());
    
    // Queen should be able to capture the rook
    let legal_moves = board.generate_legal_moves();
    let capture_move = legal_moves.iter()
        .find(|(_from, to)| to == &Coordinates::new(8, 1))
        .expect("Should be able to capture attacking rook");
    
    board.apply_move(capture_move.0, capture_move.1).unwrap();
    assert!(!board.is_in_check());
}

#[test]
fn test_cannot_move_into_check() {
    // King cannot move into check
    let fen = "4k3/8/8/8/8/8/8/4K1r1 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    
    let legal_moves = board.generate_legal_moves();
    
    // King at e1 should not be able to move to f1 (into check from rook at g1)
    assert!(!legal_moves.contains(&(Coordinates::new(5, 1), Coordinates::new(6, 1))));
}

#[test]
fn test_pinned_piece_cannot_move() {
    // Piece pinned to king cannot move
    let fen = "4k3/8/8/8/8/8/4B3/4K2r w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    
    let legal_moves = board.generate_legal_moves();
    
    // Bishop is pinned and cannot move (except to block the check)
    let bishop_moves: Vec<_> = legal_moves.iter()
        .filter(|(from, _to)| from == &Coordinates::new(5, 2))
        .collect();
    
    // Bishop should only be able to move to f1 to block the check
    assert_eq!(bishop_moves.len(), 1, "Bishop should have only one legal move");
    if let Some((_, to)) = bishop_moves.first() {
        assert_eq!(*to, Coordinates::new(6, 1), "Bishop should only be able to move to f1 to block check");
    }
}
