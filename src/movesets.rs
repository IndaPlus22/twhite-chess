//TODO going to have to pass in references in all the conditionals in all the functions.
//Creating a new temp_possible_moves list and using append() on the main possible_moves should work

/// # Arguments
///
/// 'i' - the index of the piece
///
///
/// ```
///

///
///
pub fn black_pawn_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Option<Vec<i32>>;

    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Create a vector with potential diagonal moves to make

    //Check potential moves diagonally and add the possible moves to the vector possible_moves
    for tile in potential_diagonal_moves {
        if !out_of_bounds {
            if (self.board[i - 7].unwrap().colour == White || self.board[i - 7].unwrap() == None)
                && !out_of_bounds
            {
                possible_moves = tile;
            }
            if (self.board[i - 9].unwrap().colour == White || self.board[i - 9].unwrap() == None)
                && !out_of_bounds
            {
                possible_moves = tile;
            }
        }
    }

    //Check potential moves forward and add the possible moves to the vector possible_moves
    if self.board[i - 8].unwrap().colour != Black {
        if self.board[i - 8].unwrap() == None && !out_of_bounds {
            possible_moves = tile;
        } else if self.board[i - 16].unwrap() == None && !out_of_bounds {
            possible_moves = tile;
        }
    }

    Some(possible_moves);
}
///
pub fn black_rook_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn black_knight_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Option<Vec<i32>>;

    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Create a vector with potential tiles to move to around the king
    let mut potential_moves = i - 17;
    i - 15;
    i - 10;
    i - 6;
    i + 6;
    i + 10;
    i + 15;
    i + 17;

    //TODO refactor the loop below to use for king possible moves as well
    //Check potential moves and add the possible moves to the vector possible_moves
    for tile in potential_moves {
        if !out_of_bounds {
            //Check that the tile has either a white piece or empty
            if self.board[tile].unwrap().colour == White || self.board[tile].unwrap() == None {
                possible_moves = tile;
            }
        }
    }

    Some(possible_moves)
}
///
pub fn black_bishop_possible_moves(i: i32) -> Option<Vec<i32>> {}
pub fn black_queen_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Option<Vec<i32>>;

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Add possible diagonal moves upwards left to vector.
    while (!out_of_bounds) {
        tile = tile - 9;
        // TODO refactor below conditional statements as a function
        //that takes the tile and a reference to possible_moves
        // as a parameters and adds tiles to possible_moves in this function
        if self.board[tile].unwrap().colour == White {
            possible_moves = tile;
            break;
        } else if self.board[index].unwrap() == None {
            possible_moves = tile;
        }
    }
    //TODO add the rest of diagonal move loops when test is ok

    //Add possible diagonal moves upwards right to vector.

    //Add possible diagonal moves downwards left to vector.
    while (!out_of_bounds) {
        tile = tile - 7;
        if self.board[tile].unwrap().colour == White {
            possible_moves = tile;
            break;
        } else if self.board[tile].unwrap() == None {
            possible_moves = tile;
        }
    }

    //Add possible diagonal moves downwards right to vector.

    //TODO add both vertical possible move loops

    //Add possible moves upwards vertically to vector
    while (!out_of_bounds) {
        tile = tile - 8;
        if self.board[tile].unwrap().colour == White {
            vertical_moves_up = tile;
            break;
        } else if self.board[tile].unwrap() == None {
            vertical_moves_up = tile;
        }
    }

    //Add possible moves downwards verticaly to vector

    //TODO add both horizontal possible move loops

    //Add possible moves left horizontally to vector
    while (!out_of_bounds) {
        tile = tile - 1;
        if self.board[tile].unwrap().colour == White {
            horizontal_moves_left = tile;
            break;
        } else if self.board[tile].unwrap() == None {
            horizontal_moves_left = tile;
        }
    }

    //Add possible moves right horizontally to vector

    while (!out_of_bounds) {
        tile = tile + 9;
        if self.board[tile].unwrap().colour == White {
            diagonal_moves_up_left = tile;
            break;
        } else if self.board[tile].unwrap() == None {
            diagonal_moves_up_left = tile;
        }
    }

    Some(possible_moves)
}
///
pub fn black_king_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Option<Vec<i32>>;
    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Create a vector with potential tiles to move to around the king
    let mut potential_moves = i + 7;
    i + 8;
    i + 9;
    i + 1;
    i + 1;
    i - 7;
    i - 8;
    i - 9;

    //Check potential moves and add the possible moves to the vector possible_moves
    for tile in potential_moves {
        if !out_of_bounds {
            //Check that the tile has either a white piece or empty
            if self.board[tile].unwrap().colour == White || self.board[tile].unwrap() == None {
                possible_moves = tile;
            }
        }
    }

    if possible_moves.is_empty {
        None
    }

    Some(possible_moves)
}
///
pub fn white_pawn_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Vec<i32> = Vec::new();

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    let mut possible_moves: Option<Vec<i32>>;

    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Create a vector with potential diagonal moves to make

    //Check potential moves diagonally and add the possible moves to the vector possible_moves
    for tile in potential_diagonal_moves {
        if !out_of_bounds {
            if (self.board[i + 7].unwrap().colour == Black || self.board[i + 7].unwrap() == None)
                && !out_of_bounds
            {
                possible_moves = tile;
            }
            if (self.board[i + 9].unwrap().colour == Black || self.board[i + 9].unwrap() == None)
                && !out_of_bounds
            {
                possible_moves = tile;
            }
        }
    }

    //Check potential moves forward and add the possible moves to the vector possible_moves
    if self.board[i + 8].unwrap().colour != White {
        if self.board[i + 8].unwrap() == None && !out_of_bounds {
            possible_moves = tile;
        } else if self.board[i + 16].unwrap() == None && !out_of_bounds {
            possible_moves = tile;
        }
    }
}
///
pub fn white_rook_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Vec<i32> = Vec::new();

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);
}
///
pub fn white_bishop_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Vec<i32> = Vec::new();

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    tile = i;
}
///
pub fn white_knight_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Vec<i32> = Vec::new();

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);
}
///
pub fn white_queen_possible_moves(i: i32) -> Option<Vec<i32>> {}
///
pub fn white_king_possible_moves(i: i32) -> Option<Vec<i32>> {
    let mut possible_moves: Option<Vec<i32>>;

    //Create a bool to only check tiles on the board
    let column = i % 8;
    row = i / 8;
    let out_of_bounds =
        (column - 2 < tile) && (tile < column + 2) && (row - 2 < tile) && (tile < row + 2);

    //Create a vector with potential tiles to move to around the king
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
        if !out_of_bounds {
            //Check that the tile has either a black piece or empty
            if self.board[index].unwrap().colour == Black || self.board[index].unwrap() == None {
                possible_moves.append(&mut tile);
            }
        }
    }
    Some(possible_moves)
}
