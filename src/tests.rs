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

    ///
    pub fn out_of_bounds_test() {

    }

    // TODO diagonal_moves test 
    // TODO horizontal moves tes
    // TODO vertical moves test
    // TODO parts of the pawn moveset 
    // TODO how test king movesets?
    
    ///
    pub fn parse_inputted_tile_to_index_test(tile_position: String) -> i32 {}
    ///
    pub fn black_pawn_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn black_rook_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn black_knight_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn black_bishop_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    //TODO start with testing black_king to see if appending a reference works
    ///
    pub fn black_king_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn black_queen_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_pawn_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_rook_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_bishop_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_knight_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_king_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
    ///
    pub fn white_queen_possible_moves_test(i: i32) -> Option<Vec<i32>> {}
}