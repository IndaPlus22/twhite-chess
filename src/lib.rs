use std::fmt;

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
    Black
}

pub enum PieceType {
    E, BP, BN, BB, BR, BQ, NK, WP, WN, WB, WR, WQ, WK
}

//Make a struct for each piecetype 
pub struct Piece {
    colour:Colour,
    piece_type:PieceType,
}



pub struct Game {
    /* save board, active colour, ... */
    board: [Piece;64],
    active_colour: Colour,

}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game { //0
        Game {
            /* initialise board, set active colour to white, ... */
            board:Game::board, //TODO how to initialise board
            state:GameState::InProgress, //TODO what dis
            active_colour:Colour::White,
        }
    }

    

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// param: the position to move from and move to. Returns Option
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let mut vec: Vec<String> = Vec::with_capacity(60); //TODO what is dis

        /* 
        possible_moves = get_possible_moves(from); 
        let mut move_is_possible:bool;
        
        //check if tile to move to e.g. "to" is in possible_moves
        for possible_move in 0..possible_moves.len() {
            if possible_move = to {
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
            update self::Colour
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
        
        let mut possible_moves: Vec<String> = Vec::new();
        let index = parse_inputted_tile_to_index(tile_position);

        /*     
        TODO how to find piece on the board with the index and assign to variable piece_type

        match piece_type {
            BK => possible_moves += bk_possible_moves(index);
            etc
        }

        return possible_moves;
        } */
        
        None
    }

    //Parse the letter and number in the tile name to the board index and return it
    fn parse_inputted_tile_to_index(tile_position: String) -> i32 {
        
     
        let mut col:i32;

        //Convert the letter to correct column
        match char at index 0 in tile name {
            "A" => col = 0,
            "B" => col = 1,
            "C" => col = 2,
            "D" => col = 3, 
            "E" => col = 4, 
            "F" => col = 5,
            "H" => col = 6, 
            "G" => col = 7
        } 
        

        //Convert the number in the tile name to an int 
        
        let mut row:i32;

        //could do this: let mut row = convert char at index 1 in tile_position to int.
        //Or could instead use match case if the above is complicated

        return row*8+col
    }

    //Return possible moves in a vector of strings 
    pub fn bk_possible_moves(i:i32) -> Option<Vec<String>> { //
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
    

    pub fn move_b_king() -> Option {


        /*         

        let mut temp_position = b_king_position;
        let mut previous_position = empty;

        if king_safe() == true { 
            b_king_position = temp_position;
            B_KING_POSITION = kings new position
        }

        else {
            return None
        } 
        */
    }
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