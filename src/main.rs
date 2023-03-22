use std::{io::stdin, str::FromStr};

use chess::{Board, ChessMove};

mod comms;
mod engine;
fn main() {
    //println!("feature ping=0 setboard=1 usermove=1 time=0 draw=0 reuse=0");
    let mut board = Board::default();
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line);
        let args: Vec<_> = line.split(' ').map(|s| s.trim()).collect();
        let cmd = args[0];
        match cmd {
            "new" => {
                board = Board::default();
            }
            "usermove" => {
                let usermove = ChessMove::from_str(args[1]).unwrap();
                board = board.make_move_new(usermove);
                let (bmove, eval) = engine::negamax(&board, 4);
                board = board.make_move_new(bmove.unwrap());
                dbg!(eval);
                println!("move {}", bmove.unwrap());
            }
            "xboard" => {}
            x => {
                if let Ok(usermove) = ChessMove::from_str(x) {
                    board = board.make_move_new(usermove);
                    let (bmove, eval) = engine::negamax(&board, 4);
                    board = board.make_move_new(bmove.unwrap());
                    println!("move {}", bmove.unwrap());
                }
            }
        }
    }
}
