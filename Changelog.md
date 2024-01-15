# Fenex Changelog 

## Fenex v0.1.0 (1/14/2024)

- Uploaded the project.

## Fenex v0.1.1 (1/15/2024)

### Changes:

- Switched Coordinates to 1-Index, Previously 0-Indexed.

- Renamed piece name module files, Ex. knight -> knight_piece.

## Fenex v0.1.2 (1/15/2024)

- Reverted piece names, Ex. knight_piece -> knight. (I have commitment issues)

- Introduced board struct, Can create 1D or 2D boards with an option of constructing it with      starting pieces.
  
- Minor changes to the piece struct, To allow force changes.
  - added change_coordinate.
  - added change_color.
