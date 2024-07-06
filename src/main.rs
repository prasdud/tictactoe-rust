struct Board{
    board: [[i32; 3]; 3],
}


impl Board{
    fn new() -> Board {
        Board{
            board:[
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
            ],
        }
    }
}


fn main() {
    println!("ticTacToe");
    let board = Board::new(); 
    println!("{:?}\n", board.board);
}
