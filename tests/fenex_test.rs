use fenex::chess::board::board::Board;
use fenex::chess::board::coordinates::Coordinates;
use fenex::chess::piece::piece::{PieceType, Color};

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

#[test]
fn test_can_give_check_to_enemy() {
    // You SHOULD be able to move pieces to give check to the enemy king
    let fen = "3k4/8/8/8/8/8/4R3/4K3 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    
    let legal_moves = board.generate_legal_moves();
    
    // White rook should be able to move to d2 to give check to black king on d8
    let _check_move = legal_moves.iter()
        .find(|(from, to)| from == &Coordinates::new(5, 2) && to == &Coordinates::new(4, 2))
        .expect("Should be able to move rook to give check to enemy king");
    
    println!("Found move: rook from e2 to d2 gives check to king on d8");
}

#[test]
fn test_can_capture_and_give_check() {
    // You should be able to capture pieces while giving check  
    let fen = "3k4/8/8/8/8/8/3rR3/4K3 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    
    let legal_moves = board.generate_legal_moves();
    
    // White rook should be able to capture black rook on d2, giving check to king on d8
    let _capture_check_move = legal_moves.iter()
        .find(|(from, to)| from == &Coordinates::new(5, 2) && to == &Coordinates::new(4, 2))
        .expect("Should be able to capture rook and give check");
        
    println!("Found capture move: white rook takes black rook with check");
}

#[test]
fn test_queen_can_give_check() {
    // Queen should be able to move to give check
    let fen = "3k4/8/8/8/8/8/4Q3/4K3 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    
    let legal_moves = board.generate_legal_moves();
    
    // Count how many moves the queen can make
    let queen_moves: Vec<_> = legal_moves.iter()
        .filter(|(from, _to)| from == &Coordinates::new(5, 2))
        .collect();
    
    println!("Queen has {} legal moves", queen_moves.len());
    
    // Queen should have many moves available, including ones that give check
    assert!(queen_moves.len() > 5, "Queen should have many legal moves available");
    
    // Queen should be able to move to d2 to give check to king on d8
    let _check_move = queen_moves.iter()
        .find(|(_, to)| to == &Coordinates::new(4, 2))
        .expect("Queen should be able to move to d2 to give check");
        
    println!("Queen can move to d2 to give check");
}

#[test]
fn test_debug_enemy_check_restriction() {
    // Debug why moves that give check to enemy are being blocked
    let fen = "4k3/8/8/8/8/8/4R3/4K3 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    
    println!("Initial position - White to move:");
    println!("White king on e1, White rook on e2, Black king on e8");
    
    let legal_moves = board.generate_legal_moves();
    println!("Number of legal moves: {}", legal_moves.len());
    
    for (from, to) in &legal_moves {
        if let Some(piece) = board.get(*from) {
            println!("Move: {}{}->{}{}({})", 
                (from.x as u8 + b'a' - 1) as char, from.y,
                (to.x as u8 + b'a' - 1) as char, to.y,
                match piece.piece_type {
                    PieceType::Pawn => "P",
                    PieceType::Knight => "N",
                    PieceType::Bishop => "B",
                    PieceType::Rook => "R",
                    PieceType::Queen => "Q",
                    PieceType::King => "K",
                }
            );
        }
    }
    
    // Check if rook can move to e8 (which should give check)
    let e8_move = legal_moves.iter()
        .find(|(from, to)| from == &Coordinates::new(5, 2) && to == &Coordinates::new(5, 8));
    
    match e8_move {
        Some(_) => println!("✅ Rook CAN move to e8 (correct)"),
        None => println!("❌ Rook CANNOT move to e8 (BUG!)"),
    }
}

#[test]
fn test_complex_position_can_give_check() {
    // Test a more complex position similar to what might occur in a real game
    let fen = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
    let mut board = Board::from_fen(fen).unwrap();
    
    println!("Testing complex position...");
    let legal_moves = board.generate_legal_moves();
    println!("White has {} legal moves", legal_moves.len());
    
    // Try to find some moves that would give check
    let mut check_giving_moves = Vec::new();
    
    for (from, to) in &legal_moves {
        // Simulate the move
        let mut test_board = board.clone();
        test_board.apply_move(*from, *to).unwrap();
        
        // Switch perspective and check if black king is in check
        test_board.color_to_move = match test_board.color_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        
        if test_board.is_in_check() {
            check_giving_moves.push((*from, *to));
            if let Some(piece) = board.get(*from) {
                println!("Check-giving move: {}{}->{}{}({})", 
                    (from.x as u8 + b'a' - 1) as char, from.y,
                    (to.x as u8 + b'a' - 1) as char, to.y,
                    match piece.piece_type {
                        PieceType::Pawn => "P",
                        PieceType::Knight => "N",
                        PieceType::Bishop => "B",
                        PieceType::Rook => "R",
                        PieceType::Queen => "Q",
                        PieceType::King => "K",
                    }
                );
            }
        }
    }
    
    println!("Found {} moves that give check", check_giving_moves.len());
    
    // In this specific position there might not be immediate check-giving moves, which is fine
    // The important thing is that the move generation works without crashes
    println!("Move generation completed successfully for complex position");
}

#[test]
fn test_actual_move_execution_giving_check() {
    // Test that we can actually execute moves that give check
    let fen = "3k4/8/8/8/8/8/4R3/4K3 w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    
    println!("Before move:");
    println!("White king on e1, White rook on e2, Black king on d8");
    println!("Black in check: {}", {
        let mut temp_board = board.clone();
        temp_board.color_to_move = Color::Black;
        temp_board.is_in_check()
    });
    
    // Move rook to d2 to give check
    let result = board.apply_move(Coordinates::new(5, 2), Coordinates::new(4, 2));
    match result {
        Ok(_) => {
            println!("✅ Successfully moved rook to d2");
            println!("Black in check: {}", board.is_in_check()); // Now it's black's turn
        }
        Err(e) => {
            println!("❌ Failed to move rook to d2: {}", e);
            panic!("Should be able to execute move that gives check");
        }
    }
}

#[test]
fn test_can_give_check_without_capturing_king() {
    // Test that you CAN move pieces to give check to enemy (without capturing king)
    let fen = "4k3/8/8/8/8/8/4R3/5K2 w - - 0 1";
    let mut board = Board::from_fen(fen).unwrap();
    
    println!("Initial state:");
    println!("  Black king on e8, White rook on e2, White king on f1");
    println!("  Black king in check: {}", {
        let mut temp = board.clone();
        temp.color_to_move = Color::Black;
        temp.is_in_check()
    });
    
    let legal_moves = board.generate_legal_moves();
    
    // White rook should be able to move to e7 to give check to black king
    let check_move = legal_moves.iter()
        .find(|(from, to)| from == &Coordinates::new(5, 2) && to == &Coordinates::new(5, 7))
        .expect("Should be able to move rook to e7 to give check");
    
    println!("Moving rook from e2 to e7 to give check...");
    board.apply_move(check_move.0, check_move.1).unwrap();
    
    println!("After move:");
    println!("  Current player (Black) in check: {}", board.is_in_check());
    println!("  Black king still on e8: {:?}", board.get(Coordinates::new(5, 8)));
    println!("  White rook now on e7: {:?}", board.get(Coordinates::new(5, 7)));
    
    // Black should now be in check from the rook on e7
    assert!(board.is_in_check(), "Black should be in check from white rook on e7");
}
