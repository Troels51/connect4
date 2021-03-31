
use connect::connect::*;
mod connect;

fn main() {
    let mut b: Board = Board::new();
    b.print();
    let mut gameDone = false;
    let mut currentPlayer: Player = Player::BLACK;
    while !gameDone {
        print!("{} players turn", currentPlayer);
        print!("Insert token between 0-7: ");
        let s = get_input();
        match s.parse::<u32>() {
            Ok(column) => {
                if column < COL_COUNT as u32 {
                    match b.insert(currentPlayer, column) {
                        Ok(false) => {
                            b.print();
                            currentPlayer = !currentPlayer;
                        }
                        Ok(true) => {
                            b.print();
                            println!("{} won the game!", currentPlayer);
                            gameDone = true;
                        }
                        Err(_err) => {
                            println!("Could not insert into that slot")
                        }
                    }
                } else {
                    println!("That is not a valid column");
                }
            }
            Err(err) => println!("Got invalid input"),
        };
    }
}

fn get_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
