use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub enum Colour {
    White,
    Black,
}

pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
}

pub enum PieceType {
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteKing,
    WhiteQueen,
}

pub struct Game {
    /* save board, active colour, ... */
    board: [Option<Piece>; 64],
    active_colour: Colour,
    state: GameState,
    black_king_position:i32,
    white_king_position: i32,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        //0
        use Colour::*;
        use PieceType::*;

        let white_pawn = Some(Piece {
            colour: White,
            piece_type: WhitePawn,
        });
        //TODO continue this implementation
        let white_rook = Some(Piece {
            colour: White,
            piece_type: WhiteRook,
        });
        let white_knight = Some(Piece {
            colour: White,
            piece_type: WhiteKnight,
        });
        let white_bishop = Some(Piece {
            colour: White,
            piece_type: WhiteBishop,
        });
        let white_king = Some(Piece {
            colour: White,
            piece_type: WhiteKing,
        });
        let white_queen = Some(Piece {
            colour: White,
            piece_type: WhiteQueen,
        });
        let black_pawn = Some(Piece {
            colour: Black,
            piece_type: BlackPawn,
        });
        let black_rook = Some(Piece {
            colour: Black,
            piece_type: BlackRook,
        });
        let black_knight = Some(Piece {
            colour: Black,
            piece_type: BlackKnight,
        });
        let black_bishop = Some(Piece {
            colour: Black,
            piece_type: BlackBishop,
        });
        let black_king = Some(Piece {
            colour: Black,
            piece_type: BlackKing,
        });
        let black_queen = Some(Piece {
            colour: Black,
            piece_type: BlackQueen,
        });

        Game {
            /* initialise board, set game state to in progress, set active colour to white, ... */
            //TODO which order to store pieces
            board: [],
            state: GameState::InProgress,
            active_colour: Colour::White,
            black_king_position: 4,
            white_king_position: 60,
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// param: the position to move from and move to. Returns Option
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        //code from "piece_move" function on yt
    }
}

/// Set the piece type that a pawn turns into following a promotion.
pub fn set_promotion(&mut self, piece: String) -> () {
    ()
}

//Get the current game state.//TODO what's the point of this
pub fn get_game_state(&self) -> GameState {
    self.state
}

/// If a piece is standing on the given tile, return all possible
/// new positions of that piece.
pub fn get_possible_moves(&self, tile_position: String) -> Option<Vec<String>> {
    use PieceType::*;

    let index = parse_inputted_tile_to_index(tile_position);

    //TODO how to find piece on the board with the index and assign to variable piece_type
    let piece = self.board[index].unwrap();

    if piece == None {
        None
    }

    //Return the possible moves
    Some(match piece.piece_type {
        BlackPawn => black_pawn_possible_moves(index),
        BlackRook => black_rook_possible_moves(),
        BlackKnight => black_knight_possible_moves(),
        BlackBishop => black_knight_possible_moves(),
        BlackKing => black_king_possible_moves(),
        BlackQueen => black_queen_possible_moves(),
        WhitePawn => white_pawn_possible_moves(),
        WhiteRook => white_rook_possible_moves(),
        WhiteKnight => white_knight_possible_moves(),
        WhiteBishop => white_bishop_possible_moves(),
        WhiteKing => white_king_possible_moves(),
        WhiteQueen => white_queen_possible_moves(),
    })
}

//Parse the letter and number in the tile name to the board index and return it
pub fn parse_inputted_tile_to_index(tile_position: String) -> i32 {
    let mut column: i32;

    //Convert the letter to correct column
    /*
    match case with char at index 0 in tile name {
        "A" => column = 0,
        "B" => column = 1,
        "C" => column = 2,
        "D" => column = 3,
        "E" => column = 4,
        "F" => column = 5,
        "H" => column = 6,
        "G" => column = 7
    }
    */

    //Convert the number in the tile name to an int
    let mut row: i32;

    /*
    use match case with 1 => 0 or parse string to int and subtact 1 from the result
    */

    //return the index
    return row * 8 + column;
}

/// # Arguments
///
/// 'i' - the index of the piece
///
/// ```
///  

///
pub fn black_pawn_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn black_rook_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn black_knight_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn black_bishop_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn black_king_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Vec<i32> = Vec::new();

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Create a list with potential tiles to move to around the king
    let mut potential_moves = i + 7;
    i + 8;
    i + 9;
    i + 1;
    i;
    i + 1;
    i - 7;
    i - 8;
    i - 9;

    //Check potential moves and add the possible moves to the vector possible_moves
    for tile in potential_moves {
        //Check if the king is on the tile
        if tile != i {
            if !out_of_bounds {
                //Check that the tile has either a white piece or is empty
                if self.board[index].unwrap().colour == Black || self.board[index].unwrap() == None
                {
                    possible_moves = tile;
                }
            }
        }
    }

    if possible_moves.is_empty {
        None
    }

    Some(possible_moves)
}
pub fn black_queen_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn white_pawn_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn white_rook_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn white_bishop_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn white_knight_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn white_king_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Vec<i32> = Vec::new();

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Create a list with potential tiles to move to around the king
    let mut potential_moves = i + 7;
    i + 8;
    i + 9;
    i + 1;
    i;
    i + 1;
    i - 7;
    i - 8;
    i - 9;

    //Check potential moves and add the possible moves to the vector possible_moves
    for tile in potential_moves {
        //Check if the king is on the tile
        if tile != i {
            if !out_of_bounds {
                //Check that the tile has either a white piece or is empty
                if self.board[index].unwrap().colour == White || self.board[index].unwrap() == None
                {
                    possible_moves = tile;
                }
            }
        }
    }
}
///
pub fn white_queen_possible_moves(i: i32) -> Option<Vec<i32>> {}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "") //str och String. str fixerad? Bättre använda String
    }
}
