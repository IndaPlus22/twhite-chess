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
            board: [
                
            ],
            state: GameState::InProgress,
            active_colour: Colour::White,
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// param: the position to move from and move to. Returns Option
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let mut vec: Vec<String> = Vec::with_capacity(60);

        //Check if the tile to move from is empty
        if get_possible_moves(from) == None {
            println!("{}", "Illegal move, bruh");
            None
        }

        possible_moves = get_possible_moves(from);
        let mut move_is_possible: bool;

        //check if tile to move to e.g. "to" is in possible_moves
        for possible_move in 0..possible_moves.len() {
            if possible_move == to {
                move_is_possible = true;
                break;
            } else if possible_move == possible_moves.len() {
                move_is_possible = false;
            }
        }

        if !move_is_possible {
            println!("{}", "Illegal move, bruh");
            None
        } else {
            //move piece there
        }
        //check if it is checkmate/checkmate with get_game_state?
        /*
        if checkmate {
            return GameOver
        }get_ga
        else if check {
            //update active colour
            return Check
        }
        else {
            return InProgress
        } */
    }

    /// Set the piece type that a pawn turns into following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
        ()
    }

    //Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece.
    pub fn get_possible_moves(&self, tile_position: String) -> Option<Vec<String>> {
        use PieceType::*;

        let index = parse_inputted_tile_to_index(tile_position);

        //TODO how to find piece on the board with the index and assign to variable piece_type
        let piece = self.board[index].expect("Empty square"); //TODO handle None and Some

        //Return the possible moves
        Some(match piece.piece_type {
            BlackPawn => black_pawn_possible_moves(index),
            BlackRook => todo!(),
            BlackKnight => todo!(),
            BlackBishop => todo!(),
            BlackKing => todo!(),
            BlackQueen => todo!(),
            WhitePawn => todo!(),
            WhiteRook => todo!(),
            WhiteKnight => todo!(),
            WhiteBishop => todo!(),
            WhiteKing => todo!(),
            WhiteQueen => todo!(),
        })
    }

    //Parse the letter and number in the tile name to the board index and return it
    pub fn parse_inputted_tile_to_index(tile_position: String) -> i32 {
        let mut col: i32;

        //Convert the letter to correct column
        /*
        match case with char at index 0 in tile name {
            "A" => col = 0,
            "B" => col = 1,
            "C" => col = 2,
            "D" => col = 3,
            "E" => col = 4,
            "F" => col = 5,
            "H" => col = 6,
            "G" => col = 7
        }
        */

        //Convert the number in the tile name to an int
        let mut row: i32;

        /*
        use match case with 1 => 0 osv
        */

        //return the index
        return row * 8 + col;
    }

    ///
    pub fn black_king_possible_moves(i: i32) -> Option<Vec<i32>> {
        let col = i % 8;
        row = i / 8;

        /*

        Initialize list possible_tiles_to_move_to add possible tiles somehow
        loop using index

        let mut possible_moves:Vec<i32> = Vec::new();

        loop tile in possible_tiles
            if col - 1 =< tile =< col + 1 && row -1 =< tile =< row + 1 && either is white or empty
                possible_moves = tile

        if possible_moves isempty
                    None

        Some(possible_moves)
        */
    }

    ///
    pub fn black_queen_possible_moves(i: i32) -> Option<Vec<i32>> {}

    ///
    /// # Arguments
    /// 
    /// 'i' - the index of the piece
    /// 
    /// ```
    ///  
    pub fn black_bishop_possible_moves(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn black_knight_possible_moves(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn black_rook_possible_moves(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn black_pawn_possible_moves(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_king_possible_moves(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_queen_possible_moves(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_bishop_possible_moves(i: i32) -> Option<Vec<i32>> {}

    pub fn white_knight_possible_moves(i: i32) -> Option<Vec<i32>> {}

    pub fn white_rook_possible_moves(i: i32) -> Option<Vec<i32>> {}

    pub fn white_pawn_possible_moves(i: i32) -> Option<Vec<i32>> {}
}

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

// --------------------------
// ######### TESTS ##########
// --------------------------

    /// ```
    /// let x = 5;
    /// 
    /// asser_eq!(x == 5);
    /// 
#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
