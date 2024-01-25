# Fenex Changelog

## Fenex v0.1.0 (1/14/2024)

- Uploaded the project.

## Fenex v0.1.1 (1/15/2024)

### Changes

- Switched Coordinates to 1-Index, Previously 0-Indexed.

- Renamed piece name module files, Ex. knight -> knight_piece.

## Fenex v0.1.2 (1/15/2024)

- Reverted piece names, Ex. knight_piece -> knight. (I have commitment issues)

- Introduced board struct, Can create 1D or 2D boards with an option of constructing it with starting pieces.
- Minor changes to the piece struct, To allow force changes.
  - added change_coordinate.
  - added change_color.

## Fenex v0.1.3 (1/19/2024)

- Fixed a notation coordinate conversion problem.
- Board set piece now properly moves the piece in the board.
- Refactored PieceEnum into it's separate module under chess_piece.
- Added a lot of functions to piece_enum and board.
- Changed redundant new function in the fen parser.
- Boards can now be displayed using the print_board function.

## Fenex v0.1.4 (1/21/2024)

- Removed can_capture function from piece structs.
- Changed function parameters in piece structs.
- Added more functions to the piecetype wrapper enum.
- Pieces now have is_valid_move functions.
- Board can now generate all the possible moves in the current position.
- Added has_moved value inside the king, pawn and rook struct.

## Fenex v0.1.5 (1/25/2024)

- Added fields to the board struct.
  