mod movesets;

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
    black_king_position: i32,
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
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        //Parse strings that represent tiles to indeces
        tile_to_move_from = parse_inputted_tile_to_index(from);
        tile_to_move_to = parse_inputted_tile_to_index(to);

        self.board[tile_to_move_to] = self.board[tile_to_move_from];
    }

    //Parse the letter and number in the tile name to the board index and return it
    pub fn parse_inputted_tile_to_index(tile: String) -> i32 {
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
    pub fn get_possible_moves(&self, tile: String) -> Option<Vec<String>> {
        use PieceType::*;

        let index = parse_inputted_tile_to_index(tile);

        //TODO how to find piece on the board with the index and assign to variable piece_type
        let piece = self.board[index].unwrap();

        if piece == None {
            None
        }

        //Return the possible moves
        Some(match piece.piece_type {
            BlackPawn => black_pawn_possible_moves(index),
            BlackRook => black_rook_possible_moves(index),
            BlackKnight => black_knight_possible_moves(index),
            BlackBishop => black_knight_possible_moves(index),
            BlackKing => black_king_possible_moves(index),
            BlackQueen => black_queen_possible_moves(index),
            WhitePawn => white_pawn_possible_moves(index),
            WhiteRook => white_rook_possible_moves(index),
            WhiteKnight => white_knight_possible_moves(index),
            WhiteBishop => white_bishop_possible_moves(index),
            WhiteKing => white_king_possible_moves(index),
            WhiteQueen => white_queen_possible_moves(index),
        })
    }
}

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
