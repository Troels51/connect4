#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use std::{error::Error, fs::File, io::{self, BufRead}, path::{Path, PathBuf}, str::FromStr};
    use test::Bencher;
    use connect4::{self, connect::connect::Board};
    use csv::Reader;
    use serde::{Deserialize, Deserializer, de::{self, Visitor}};
    use std::fmt;



    #[derive(Debug, Deserialize)]
    struct Record {
        board : Board,
        score : i32,
    }
    fn test_from_file(path: PathBuf) -> Result<(), Box<dyn Error >>{
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_path(path);
            
        match rdr {
            Ok(mut result) => {
                for value in result.deserialize() {
                    let record : Record = value?;
                    println!("board: {}, current_player {}", record.board, record.board.current_player);
                    let score = record.board.negamax();
                    assert_eq!(score, record.score, "This board {} should score {} but scored {}", record.board, record.score, score);
                }
            }
            Err(_err) => {assert!(false);}
        }
        return Ok(());
    }
    #[test]
    fn easy_boards_test(){
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L3_R1");
        test_from_file(test_data).unwrap();
    }
    #[test]
    fn check_win_test() {
        let mut b : Board = Board::from_str("2252576253462244111563365343671351441").unwrap();
        assert!(!b.check_move_for_win(6));
        b.print();
        b.play(6);
        b.print();
        assert!(b.check_move_for_win(5));
        b.print();
        assert!(false);

    }
    #[bench]
    fn easy_boards(b : &mut Bencher) {
        b.iter(|| 2+2);
    }
}