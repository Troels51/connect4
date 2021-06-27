use connect4::connect::board::{*, self};

fn main() {
    let _b: board::Board = board::Board::new();
    // let mut game_done: bool = false;
    // while !game_done {
    //     print!("{} players turn", b.current_player);
    //     print!("Insert token between 0-7: ");
    //     let s = get_input();
    //     match s.parse::<u32>() {
    //         Ok(column) => {
    //             if column < COL_COUNT as u32 {
    //                 b.play(column);

    //                 match b.checkwin(latest_token, player) {
    //                     Ok(false) => {
    //                         b.print();
    //                     }
    //                     Ok(true) => {
    //                         b.print();
    //                         println!("{} won the game!", b.current_player);
    //                         game_done = true;
    //                     }
    //                     Err(_err) => {
    //                         println!("Could not insert into that slot")
    //                     }
    //                 }
    //             } else {
    //                 println!("That is not a valid column");
    //             }
    //         }
    //         Err(_err) => println!("Got invalid input"),
    //     };
    // }
}

// fn get_input() -> String {
//     use std::io::{stdin, stdout, Write};
//     let mut s = String::new();
//     let _ = stdout().flush();
//     stdin()
//         .read_line(&mut s)
//         .expect("Did not enter a correct string");
//     if let Some('\n') = s.chars().next_back() {
//         s.pop();
//     }
//     if let Some('\r') = s.chars().next_back() {
//         s.pop();
//     }
//     s
// }
