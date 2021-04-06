
use connect4::connect::connect::*;

fn main() {
    let mut b: Board = Board::new();
    b.print();
    let mut game_done: bool = false;
    let mut current_player: Player = Player::BLACK;
    while !game_done {
        print!("{} players turn", current_player);
        print!("Insert token between 0-7: ");
        let s = get_input();
        match s.parse::<u32>() {
            Ok(column) => {
                if column < COL_COUNT as u32 {
                    match b.insert(current_player, column) {
                        Ok(false) => {
                            b.print();
                            current_player = !current_player;
                        }
                        Ok(true) => {
                            b.print();
                            println!("{} won the game!", current_player);
                            game_done = true;
                        }
                        Err(_err) => {
                            println!("Could not insert into that slot")
                        }
                    }
                } else {
                    println!("That is not a valid column");
                }
            }
            Err(_err) => println!("Got invalid input"),
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
