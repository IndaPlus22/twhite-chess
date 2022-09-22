use std::{fmt, io::Empty};

// (1234b) means row1, column2 moves to row3, column4 and captures b
//and a space owuld represent no capture

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub enum Colour {
    White,
    Black,
    NoColour,
}

pub struct Piece {
    colour:Colour,
    piece_type:PieceType,
}

pub enum PieceType {
    Empty, BlackPawn, BlackRook, BlackKnight, BlackBishop, BlackKing, BlackQueen, 
    WhitePawn, WhiteRook, WhiteKnight, WhiteBishop, WhiteKing, WhiteQueen,
}


pub struct Game {
    /* save board, active colour, ... */
    board: [Piece;64],
    active_colour: Colour,
    state:GameState,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game { //0
        use Colour::*;
        use PieceType::*;

        let white_pawn = Piece {
            colour:White,
            piece_type:BP 
        };
        //TODO continue this implementation
        let white_rook = Piece {
            colour:White,
            piece_type:BP 
        }; 
        let white_pawn = Piece {
            colour:White,
            piece_type:BP 
        }; 
        let white_pawn = Piece {
            colour:White,
            piece_type:BP 
        }; 
        let white_pawn = Piece {
            colour:White,
            piece_type:BP 
        }; 
        let white_pawn = Piece {
            colour:White,
            piece_type:BP 
        };
        let empty_tile: Piece = Piece {
            colour:Empty,
            piece_type:E,
        };

        Game {
            /* initialise board, set active colour to white, ... */
            
            //TODO fill board with correct piecetype
            board: [
    
            ],
            state:GameState::InProgress,
            active_colour:Colour::White,   
        }
    }

    

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// param: the position to move from and move to. Returns Option
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let mut vec: Vec<String> = Vec::with_capacity(60); //TODO what is dis

        /* 
        if get_possible_moves() == None {
            println!("{}", "Illegal move, bruh");
            None
        }

        possible_moves = get_possible_moves(from); 
        let mut move_is_possible:bool;
        
        //check if tile to move to e.g. "to" is in possible_moves
        for possible_move in 0..possible_moves.len() {
            if possible_move == to {
                move_is_possible = true;
                continue;
            }
            else if possible_move == possible_moves.len() {
                move_is_possible = false;
            }
        }

        if !move_is_possible {
            println!("{}", "Illegal move, bruh");
            None
        }
        else {
            move piece there
        }
        check if it is checkmate/checkmate with get_game_state?
            if checkmate {
                return GameOver
            }
            else if check {
                update active colour
                return Check
            }
            else {
                return InProgress
            }
        */ 
    }

    /// Set the piece type that a peasant becames following a promotion.
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
        
        let mut possible_moves: Vec<String> = Vec::new();
        let index = parse_inputted_tile_to_index(tile_position);
        
        //TODO how to find piece on the board with the index and assign to variable piece_type
        let piece: Piece = self.board[index];     

        match piece.piece_type {
            Empty => None,
            BlackPawn => possible_moves = bk_possible_moves(index),
        }

        return possible_moves
    }
        
        None
    }

    //Parse the letter and number in the tile name to the board index and return it
    fn parse_inputted_tile_to_index(tile_position: String) -> i32 {

        let mut col:i32;

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
        let mut row:i32;

        /* 
        could do this: let mut row = convert char at index 1 in tile_position to int.
        Or could instead use match case if the above is complicated 
        */

        //return the index
        return row*8+col

    }

    //Return possible moves in a vector of strings 
    pub fn bk_possible_moves(i:i32) -> Option<Vec<i32>> { //
        /* 
        //TODO figure out logic for if king is at sides or corner. 
         
        Initialize list possible_tiles_to_move_to add possible tiles somehow
            loop using index

        Initialize possible_moves 
        


        loop tile in possible_tiles 
            if tile != 4 {
                if 

                if tile has empty space or white piece && !(tile == i%8 || tile == i/8) {
                    add to list of possible moves
                }
        
        return vec with indexes of possible moves 
        */

        let mut possible_moves:Vec<String> = Vec::new();
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