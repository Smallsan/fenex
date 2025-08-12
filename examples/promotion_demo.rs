use fenex::chess::{
    board::{board::Board, coordinates::Coordinates},
    piece::piece::{PieceType},
};

fn main() {
    // Create a position where white pawn is about to promote
    let fen = "8/P7/8/8/8/8/8/8 w - - 0 1";
    
    println!("Fenex Pawn Promotion Demo");
    println!("========================");
    println!("Starting position: {}", fen);
    println!();
    
    // Demo 1: Default promotion (Queen)
    let mut board1 = Board::from_fen(fen).unwrap();
    board1.apply_move(Coordinates::new(1, 7), Coordinates::new(1, 8)).unwrap();
    println!("1. Default promotion (Queen): {:?}", 
             board1.get(Coordinates::new(1, 8)).unwrap().piece_type);
    
    // Demo 2: Specific promotion to Rook
    let mut board2 = Board::from_fen(fen).unwrap();
    board2.promote_to_rook(Coordinates::new(1, 7), Coordinates::new(1, 8)).unwrap();
    println!("2. Promote to Rook: {:?}", 
             board2.get(Coordinates::new(1, 8)).unwrap().piece_type);
    
    // Demo 3: Specific promotion to Bishop
    let mut board3 = Board::from_fen(fen).unwrap();
    board3.promote_to_bishop(Coordinates::new(1, 7), Coordinates::new(1, 8)).unwrap();
    println!("3. Promote to Bishop: {:?}", 
             board3.get(Coordinates::new(1, 8)).unwrap().piece_type);
    
    // Demo 4: Specific promotion to Knight
    let mut board4 = Board::from_fen(fen).unwrap();
    board4.promote_to_knight(Coordinates::new(1, 7), Coordinates::new(1, 8)).unwrap();
    println!("4. Promote to Knight: {:?}", 
             board4.get(Coordinates::new(1, 8)).unwrap().piece_type);
    
    // Demo 5: Using apply_move_with_promotion
    let mut board5 = Board::from_fen(fen).unwrap();
    board5.apply_move_with_promotion(
        Coordinates::new(1, 7), 
        Coordinates::new(1, 8), 
        Some(PieceType::Rook)
    ).unwrap();
    println!("5. Using apply_move_with_promotion (Rook): {:?}", 
             board5.get(Coordinates::new(1, 8)).unwrap().piece_type);
    
    // Demo 6: Error handling for invalid promotion
    let mut board6 = Board::from_fen(fen).unwrap();
    let result = board6.apply_move_with_promotion(
        Coordinates::new(1, 7), 
        Coordinates::new(1, 8), 
        Some(PieceType::King)  // Invalid promotion piece
    );
    println!("6. Invalid promotion attempt: {:?}", result);
    
    println!();
    println!("All promotion options are now available!");
    println!("No more forced Queen promotions!");
}
