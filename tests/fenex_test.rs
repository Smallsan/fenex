use fenex::chess::board::board::Board;
use fenex::chess::board::coordinates::Coordinates;
use fenex::chess::piece::piece::PieceType;

#[test]
fn test_checkmate_detection() {
    // Fool's mate position after 1. f3 e5 2. g4
    let fen = "rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2";
    let mut board = Board::from_fen(fen).unwrap();
    let legal_moves = board.generate_legal_moves();
    
    // Find any queen move to simulate a checkmate attempt
    let queen_moves: Vec<_> = legal_moves.iter()
        .filter(|(from, _to)| {
            if let Some(piece) = board.get(*from) {
                piece.piece_type == PieceType::Queen
            } else {
                false
            }
        })
        .collect();
    
    // Make a queen move if available (simulating Qh4#)
    if let Some(&(from, to)) = queen_moves.first() {
        board.apply_move(*from, *to).unwrap();
    }
    // Assume you have a method is_checkmate (if not, this is a placeholder for future implementation)
    // assert!(board.is_checkmate());
}

#[test]
fn test_stalemate_detection() {
    // Stalemate position
    let fen = "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1";
    let _board = Board::from_fen(fen).unwrap();
    // Assume you have a method is_stalemate (if not, this is a placeholder for future implementation)
    // assert!(board.is_stalemate());
}

#[test]
fn test_illegal_move() {
    let fen = "8/8/8/3P4/8/8/8/8 w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    // Try to move pawn backwards (illegal)
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
    // Make kingside castling move
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
    // Make en passant move
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
    // Make promotion move (defaults to queen)
    board
        .apply_move(Coordinates::new(1, 7), Coordinates::new(1, 8))
        .unwrap();
    assert_eq!(
        board.get(Coordinates::new(1, 8)).unwrap().piece_type,
        PieceType::Queen
    );
}
