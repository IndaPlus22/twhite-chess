fn main() {
    impl<T: Clone> Array2D<T> {}

    pub fn from_rows(elements: &[Vec<T>]) -> Self {}

    //Unfinished visual board representation to test printing the board
    //correctly
    let board = [
        "br", "bn", "bb", "bq", "bk", "bb", "bn", "br", "bp", "bp", "bp", "bp", "bp", "bp", "bp",
        "bp", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e",
        "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "e", "wp", "wp",
        "wp", "wp", "wp", "wp", "wp", "wp", "wr", "wn", "wb", "wq", "wk", "wb", "wn", "wr",
    ];

    print_board(board);

    //TODO implement an array with board data
}

fn print_board(array: board) {
    //prints the initital board
    //TODO fix this to print each line correctly with rank and file next to the board
    for piece in board {
        println!("{}", piece)
    }
}
