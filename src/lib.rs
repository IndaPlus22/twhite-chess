use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
}

#[derive(Copy, Clone, Debug, PartialEq)]

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
    black_king_position: usize,
    white_king_position: usize,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        use Colour::*;
        use PieceType::*;

        let white_pawn = Some(Piece {
            colour: White,
            piece_type: WhitePawn,
        });
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
            /* initialise board, set game state to in progress, set active colour to white,
            set both kings positions */
            board: [
                white_rook,
                white_knight,
                white_bishop,
                white_queen,
                white_king,
                white_bishop,
                white_knight,
                white_rook,
                white_pawn,
                white_pawn,
                white_pawn,
                white_pawn,
                white_pawn,
                white_pawn,
                white_pawn,
                white_pawn,
                None,
                black_pawn,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                black_pawn,
                black_pawn,
                black_pawn,
                black_pawn,
                black_pawn,
                black_pawn,
                black_pawn,
                black_pawn,
                black_rook,
                black_knight,
                black_bishop,
                black_queen,
                black_king,
                black_bishop,
                black_knight,
                black_rook,
            ],
            state: GameState::InProgress,
            active_colour: Colour::White,
            black_king_position: 60,
            white_king_position: 4,
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and
    /// TODO return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) {
        //Parse strings that represent tiles to indeces
        let tile_to_move_from = Self::parse_inputted_tile_to_index(from);
        let tile_to_move_to = Self::parse_inputted_tile_to_index(to);

        self.board[tile_to_move_to] = self.board[tile_to_move_from];
        self.board[tile_to_move_from] = None;

        //TODO implement with get_possible_moves 
    }

    //Parse the letter and number in the tile name to the board index and return it
    pub fn parse_inputted_tile_to_index(tile_string: String) -> usize {
        let mut column = 0;
        //TODO rank och file ist för row och column

        //Convert the letter to correct column index
        if tile_string.find("A") == Some(0) {
            column = 0;
        } else if tile_string.find("B") == Some(0) {
            column = 1;
        } else if tile_string.find("C") == Some(0) {
            column = 2;
        } else if tile_string.find("D") == Some(0) {
            column = 3;
        } else if tile_string.find("E") == Some(0) {
            column = 4;
        } else if tile_string.find("F") == Some(0) {
            column = 5;
        } else if tile_string.find("G") == Some(0) {
            column = 6;
        } else if tile_string.find("H") == Some(0) {
            column = 7;
        }

        let mut row: usize = 0;

        //Convert the number in the tile name to correct row index
        if tile_string.find("1") == Some(1) {
            row = 0;
        } else if tile_string.find("2") == Some(1) {
            row = 1;
        } else if tile_string.find("3") == Some(1) {
            row = 2;
        } else if tile_string.find("4") == Some(1) {
            row = 3;
        } else if tile_string.find("5") == Some(1) {
            row = 4;
        } else if tile_string.find("6") == Some(1) {
            row = 5;
        } else if tile_string.find("7") == Some(1) {
            row = 6;
        } else if tile_string.find("8") == Some(1) {
            row = 7;
        }

        //Return the index
        row * 8 + column
    }

    /// Set the piece type that a pawn turns into following a promotion. TODO promotion
    pub fn set_promotion(&mut self, piece: String) -> () {
        ()
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece.
    pub fn get_possible_moves(&self, tile: String) -> Option<Vec<usize>> {
        use PieceType::*;

        //Parse the string tile to an index i
        let i = Self::parse_inputted_tile_to_index(tile);

        if self.board[i].is_none() {
            return None;
        }

        //Get the piece on the tile
        let piece = self.board[i].as_ref().unwrap();

        //Return the possible moves
        match piece.piece_type {
            BlackPawn => self.black_pawn_possible_moves(i),
            BlackRook => self.black_rook_possible_moves(i),
            BlackKnight => self.black_knight_possible_moves(i),
            BlackBishop => self.black_knight_possible_moves(i),
            BlackKing => self.black_king_possible_moves(i),
            BlackQueen => self.black_queen_possible_moves(i),
            WhitePawn => self.white_pawn_possible_moves(i),
            WhiteRook => self.white_rook_possible_moves(i),
            WhiteKnight => self.white_knight_possible_moves(i),
            WhiteBishop => self.white_bishop_possible_moves(i),
            WhiteKing => self.white_king_possible_moves(i),
            WhiteQueen => self.white_queen_possible_moves(i),
        }
    }

    /// # Arguments
    ///
    /// 'i' - the index of the piece
    ///
    ///
    /// ```
    ///

    ///
    ///
    pub fn black_pawn_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        let mut possible_moves: Vec<usize> = Vec::new();
        //TODO fixa logiken med kolumner
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        let potential_diagonal_moves: Vec<usize> = vec![i - 7, i - 9];

        //Create a vector with potential diagonal moves to make
        for tile in potential_diagonal_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row < index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                //TODO hur hitta vit eller vart ska man ha allt
                if (self.board[tile].as_ref().unwrap().colour == White
                    || self.board[tile].is_none())
                    && !out_of_bounds
                {
                    possible_moves.push(tile);
                }
                if (self.board[tile].as_ref().unwrap().colour == White
                    || self.board[tile].is_none())
                    && !out_of_bounds
                {
                    possible_moves.push(tile);
                }
            }
        }

        let forward_moves: Vec<usize> = vec![i - 8, i - 16];
        //
        for tile in forward_moves {
            //Check potential moves forward and add the possible moves to the vector possible_moves
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row < index_column - 1)
                || (index_column + 1 < tile_column);
            if self.board[tile].as_ref().unwrap().colour != Black {
                if self.board[i - 8].is_none() && !out_of_bounds {
                    possible_moves.push(tile);
                } else if self.board[i - 16].is_none() && !out_of_bounds && index_row == 6 {
                    possible_moves.push(tile);
                }
            }
        }

        Some(possible_moves)
    }

    ///
    pub fn black_rook_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        None
    }

    ///
    pub fn black_knight_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        let mut possible_moves: Vec<usize> = Vec::new();

        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        //Create a vector with potential tiles to move to around the king
        let potential_moves: Vec<usize> =
            vec![i - 17, i - 15, i - 10, i - 6, i + 6, i + 10, i + 15, i + 17];

        //TODO refactor the loop below to use for king possible moves as well
        //Check potential moves and add the possible moves to the vector possible_moves
        for tile in potential_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row < index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                //Check that the tile has either a white piece or empty
                if self.board[tile].as_ref().unwrap().colour == White || self.board[tile].is_none()
                {
                    possible_moves.push(tile);
                }
            }
        }

        Some(possible_moves)
    }
    ///
    pub fn black_bishop_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        None
    }
    pub fn black_queen_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        let mut possible_moves: Vec<usize> = Vec::new();

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;
        let mut tile: usize = i;

        let mut tile_column: i32 = tile as i32 % 8;
        let mut tile_row = tile as i32 / 8;

        //Add possible diagonal moves upwards left to vector.
        //TODO fixa så att out of bounds är utanför while eller tänk om
        let mut out_of_bounds = (tile_column < index_column - 1)
            || (index_column + 1 < tile_column)
            || (tile_row <= index_column - 1)
            || (index_column + 1 < tile_column);
        while !out_of_bounds {
            tile -= 9;
            // TODO refactor below conditional statements as a function
            //that takes the tile and a reference to possible_moves
            // as a parameters and adds tiles to possible_moves in this function
            if self.board[tile].as_ref().unwrap().colour == White {
                possible_moves.push(tile);
                break;
            } else if self.board[tile].is_none() {
                possible_moves.push(tile);
            }
            out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
            tile_column = tile as i32 % 8;
            tile_row = tile as i32 / 8;
        }
        //TODO add the rest of diagonal move loops when test is ok

        //Add possible diagonal moves upwards right to vector.

        /*  //Add possible diagonal moves downwards left to vector.
        while !out_of_bounds {
            tile -= 7;
            if self.board[tile  ].as_ref().unwrap().colour == White {
                possible_moves.push(tile);
                break;
            } else if self.board[tile  ].is_none() {
                possible_moves.push(tile);
            }
            out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
        }

        //Add possible diagonal moves downwards right to vector.

        //TODO add both vertical possible move loops

        //Add possible moves upwards vertically to vector
        while !out_of_bounds {
            tile = tile - 8;
            if self.board[tile  ].as_ref().unwrap().colour == White {
                possible_moves.push(tile);
                break;
            } else if self.board[tile  ].is_none() {
                possible_moves.push(tile);
            }
            out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
        }

        //Add possible moves downwards verticaly to vector

        //TODO add both horizontal possible move loops

        //Add possible moves left horizontally to vector
        while !out_of_bounds {
            tile = tile - 1;
            if self.board[tile  ].as_ref().unwrap().colour == White {
                possible_moves.push(tile);
                break;
            } else if self.board[tile  ].is_none() {
                possible_moves.push(tile);
            }
            out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
        }

        //Add possible moves right horizontally to vector

        while !out_of_bounds {
            tile = tile + 9;
            if self.board[tile  ].as_ref().unwrap().colour == White {
                possible_moves = tile;
                break;
            } else if self.board[tile  ].is_none() {
                possible_moves = tile;
            }
            out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
        } */

        Some(possible_moves)
    }
    ///
    pub fn black_king_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        let mut possible_moves: Vec<usize> = Vec::new();
        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        //Create a vector with potential tiles to move to around the king
        let potential_moves: Vec<usize> =
            vec![i + 7, i + 8, i + 9, i + 1, i - 1, i - 7, i - 8, i - 9];

        //Check potential moves and add the possible moves to the vector possible_moves
        for tile in potential_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                //Check that the tile has either a white piece or empty
                if self.board[tile].as_ref().unwrap().colour == White || self.board[tile].is_none()
                {
                    possible_moves.push(tile);
                }
            }
        }

        if possible_moves.is_empty() {
            return None;
        }

        Some(possible_moves)
    }
    ///
    pub fn white_pawn_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        let mut possible_moves: Vec<usize> = Vec::new();

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        let mut possible_moves: Vec<usize> = Vec::new();
        let potential_diagonal_moves: Vec<usize> = vec![i + 7, i + 9];

        //Check potential moves diagonally and add the possible moves to the vector possible_moves
        for tile in potential_diagonal_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                if (game.board[i + 7].as_ref().unwrap().colour == Black
                    || game.board[i + 7].is_none())
                    && !out_of_bounds
                {
                    possible_moves.push(tile);
                }
                if (game.board[i + 9].as_ref().unwrap().colour == Black
                    || game.board[i + 9].is_none())
                    && !out_of_bounds
                {
                    possible_moves.push(tile);
                }
            }
        }
        Some(possible_moves)
    }
    ///
    pub fn white_rook_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;

        let mut possible_moves: Vec<usize> = Vec::new();

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        Some(possible_moves)
    }
    ///
    pub fn white_bishop_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        let mut possible_moves: Vec<usize> = Vec::new();

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        Some(possible_moves)
    }
    ///
    pub fn white_knight_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        let mut possible_moves: Vec<usize> = Vec::new();

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        Some(possible_moves)
    }
    ///
    pub fn white_queen_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        None
    }
    ///
    pub fn white_king_possible_moves(&self, i: usize) -> Option<Vec<usize>> {
        use Colour::*;
        let mut possible_moves: Vec<usize> = Vec::new();

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        //Create a vector with potential tiles to move to around the king
        let potential_moves: Vec<usize> =
            vec![i + 7, i + 8, i + 9, i + 1, i - 1, i - 7, i - 8, i - 9];

        //Check potential moves and add the possible moves to the vector possible_moves
        for tile in potential_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row < index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                //Check that the tile has either a black piece or empty
                //TODO figure out why self.board[tile  ].as_ref().unwrap().colour doesn't get the colour
                if self.board[tile].is_none() || self.board[tile].as_ref().unwrap().colour == Black
                {
                    possible_moves.push(tile);
                }
            }
        }
        Some(possible_moves)
    }
}

//TODO going to have to pass in references in all the conditionals in all the functions.
//Creating a new temp_possible_moves list and using push() on the main possible_moves should work

/// TODO Implement print routine for Game.
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
    use super::*;

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

        assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn active_colour_white_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.active_colour, Colour::White);
    }

    #[test]
    fn king_position_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.white_king_position, 4);
        assert_eq!(game.black_king_position, 60)
    }

    #[test]
    fn colour_of_piece_and_piece_type() {
        use Colour::*;
        use PieceType::*;

        let game = Game::new();

        println!("{:?}", game);

        //Get the black king piece
        let piece = game.board[60].as_ref().unwrap();

        assert_eq!(piece.colour, Black);
        assert_eq!(piece.piece_type, BlackKing);
    }

    #[test]
    ///
    fn parse_inputted_tile_to_index_test() {
        let game = Game::new();

        println!("{:?}", game);

        let tile_position = String::from("A1");
        let index1 = Game::parse_inputted_tile_to_index(tile_position);

        let tile_position = String::from("H8");
        let index2 = Game::parse_inputted_tile_to_index(tile_position);

        assert_eq!(index1, 0);
        assert_eq!(index2, 63);
    }

    fn correct_row_and_column() {
        let i: i32 = 16;

        let index_column: i32 = i % 8;
        let index_row: i32 = i / 8;

        let out_of_bounds_tile: i32 = 15;

        let tile_column: i32 = out_of_bounds_tile % 8;
        let tile_row: i32 = out_of_bounds_tile / 8;

        assert_eq!(index_column, 0);
        assert_eq!(index_row, 2);
        assert_eq!(tile_column, 7);
        assert_eq!(tile_row, 1);
    }

    #[test]
    fn out_of_bounds_test() {
        let i: i32 = 16;

        let index_column: i32 = i % 8;
        let index_row: i32 = i / 8;

        let out_of_bounds_tile: i32 = 15;

        let tile_column: i32 = out_of_bounds_tile as i32 % 8;
        let tile_row: i32 = out_of_bounds_tile as i32 / 8;

        let out_of_bounds = (tile_column < index_column - 1)
            || (index_column + 1 < tile_column)
            || (tile_row < index_column - 1)
            || (index_column + 1 < tile_column);
        assert!(out_of_bounds);
    }

/*     #[test]
    fn make_move_test() {

        let from: String = String::from("A2");
        let to: String = String::from("A3");

        //Parse strings that represent tiles to indeces
        let tile_to_move_from = Self::parse_inputted_tile_to_index(from);
        let tile_to_move_to = Self::parse_inputted_tile_to_index(to);

        game.board[tile_to_move_to] = game.board[tile_to_move_from];
        game.board[tile_to_move_from] = None;
    } */

    #[test]
    ///
    fn white_pawn_possible_moves_test() {
        use Colour::*;

        let game = Game::new();

        println!("{:?}", game);

        let i = 8;

        let mut possible_moves: Vec<usize> = Vec::new();

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        let mut possible_moves: Vec<usize> = Vec::new();
        let potential_diagonal_moves: Vec<usize> = vec![i + 7, i + 9];

        //Check potential moves diagonally and add the possible moves to the vector possible_moves
        for tile in potential_diagonal_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                if (game.board[i + 7].as_ref().unwrap().colour == Black
                    || game.board[i + 7].is_none())
                    && !out_of_bounds
                {
                    possible_moves.push(tile);
                }
                if (game.board[i + 9].as_ref().unwrap().colour == Black
                    || game.board[i + 9].is_none())
                    && !out_of_bounds
                {
                    possible_moves.push(tile);
                }
            }
        }

        //Check potential moves forward and add the possible moves to the vector possible_moves
        if game.board[i+8].is_none() {
            possible_moves.push(i+8);
            if game.board[i + 16].is_none() && index_row == 1 {
                possible_moves.push(i+16);
            }
        }

        possible_moves.sort();

        let actual_possible_moves: Vec<usize> = vec![16, 17, 24];

        assert_eq!(actual_possible_moves, possible_moves);
                
    }





    ///
    #[test]
    fn white_king_possible_moves_test() {
        use Colour::*;

        let game = Game::new();

        println!("{:?}", game);

        let mut possible_moves: Vec<usize> = Vec::new();
        let i: usize = 16;

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        //Create a vector with potential tiles to move to around the king
        let potential_moves: Vec<usize> =
            vec![i + 7, i + 8, i + 9, i + 1, i - 1, i - 7, i - 8, i - 9];

        //Check potential moves and add the possible moves to the vector possible_moves
        for tile in potential_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row < index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                //Check that the tile has either a black piece or empty
                if game.board[tile].is_none() || game.board[tile].as_ref().unwrap().colour == Black
                {
                    possible_moves.push(tile);
                }
            }
        }
        let actual_possible_moves: Vec<usize> = vec![i + 8, i + 9, i + 1];

        assert_eq!(actual_possible_moves, possible_moves);
    }

    ///
    #[test]
    fn black_king_possible_moves_test() {
        use Colour::*;

        let game = Game::new();

        println!("{:?}", game);

        let mut possible_moves: Vec<usize> = Vec::new();
        let i: usize = 40;

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;

        //Create a vector with potential tiles to move to around the king
        let potential_moves: Vec<usize> =
            vec![i + 7, i + 8, i + 9, i + 1, i - 1, i - 7, i - 8, i - 9];

        //Check potential moves and add the possible moves to the vector possible_moves
        for tile in potential_moves {
            let mut tile_column: i32 = tile as i32 % 8;
            let mut tile_row: i32 = tile as i32 / 8;
            let mut out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row < index_column - 1)
                || (index_column + 1 < tile_column);
            if !out_of_bounds {
                //Check that the tile has either a black piece or empty
                //TODO figure out why self.board[tile  ].as_ref().unwrap().colour doesn't get the colour
                if game.board[tile].is_none() || game.board[tile].as_ref().unwrap().colour == White
                {
                    possible_moves.push(tile);
                }
            }
        }
        let actual_possible_moves: Vec<usize> = vec![i + 1, i - 7, i - 8];

        assert_eq!(actual_possible_moves, possible_moves);
    }

    ///
    #[test]
    fn black_queen_diagonal_moves_test() {
        use Colour::*;
        use PieceType::*;

        let mut game = Game::new();

        let white_pawn = Some(Piece {
            colour: White,
            piece_type: WhitePawn,
        });
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

        println!("{:?}", game);

        game.board = [
            white_rook,
            white_knight,
            white_bishop,
            white_queen,
            white_king,
            white_bishop,
            white_knight,
            white_rook,
            white_pawn,
            white_pawn,
            white_pawn,
            white_pawn,
            white_pawn,
            white_pawn,
            white_pawn,
            white_pawn,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            black_pawn,
            black_pawn,
            black_pawn,
            black_pawn,
            black_pawn,
            black_pawn,
            black_pawn,
            black_pawn,
            black_rook,
            black_knight,
            black_bishop,
            black_queen,
            black_king,
            black_bishop,
            black_knight,
            black_rook,
        ];

        let mut possible_moves: Vec<usize> = Vec::new();

        let i = 27;

        //Create a bool to only check tiles on the board
        let mut index_column: i32 = i as i32 % 8;
        let mut index_row: i32 = i as i32 / 8;
        let mut tile: usize = i;

        let mut tile_column: i32 = tile as i32 % 8;
        let mut tile_row = tile as i32 / 8;

        //Add possible diagonal moves upwards left to vector.
        //TODO fixa så att out of bounds är utanför while eller tänk om
        let mut out_of_bounds = (tile_column < index_column - 1)
            || (index_column + 1 < tile_column)
            || (tile_row <= index_column - 1)
            || (index_column + 1 < tile_column);
        while !out_of_bounds {
            tile -= 9;
            // TODO refactor below conditional statements as a function
            //that takes the tile and a reference to possible_moves
            // as a parameters and adds tiles to possible_moves in this function
            if game.board[tile].as_ref().unwrap().colour == White {
                possible_moves.push(tile);
                break;
            } else if game.board[tile].is_none() {
                possible_moves.push(tile);
            }
            out_of_bounds = (tile_column < index_column - 1)
                || (index_column + 1 < tile_column)
                || (tile_row <= index_column - 1)
                || (index_column + 1 < tile_column);
            tile_column = tile as i32 % 8;
            tile_row = tile as i32 / 8;
        }
        let actual_possible_moves: Vec<usize> = vec![18, 20, 34, 36, 41, 55];

        assert_eq!(actual_possible_moves, possible_moves);
    }
    /*
        TODO diagonal_moves test
        TODO makemove test
        TODO horizontal moves tes
        TODO vertical moves test
        TODO parts of the pawn moveset
        TODO how test king movesets?


        ///
        fn black_pawn_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
        ///
        fn black_rook_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
        ///
        fn black_knight_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
        ///
        fn black_bishop_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
        ///
        fn white_rook_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
        ///
        fn white_bishop_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
        ///
        fn white_knight_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
        ///
        fn white_queen_possible_moves_test(i: usize) -> Option<Vec<usize>> {}
    */
}
