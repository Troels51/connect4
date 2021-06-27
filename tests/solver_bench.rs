#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use std::{error::Error, fs::File, io::{self, BufRead}, path::{Path, PathBuf}, str::FromStr};
    use test::Bencher;
    use connect4::{self, connect::{board::Board, solver::Solver}};
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
                    let mut solver : Solver = Solver::new();
                    let record : Record = value?;
                    let score = solver.solve(record.board);
                    assert_eq!(score, record.score, "The score should {} but scored {}, for this board {}", record.score, score, record.board);
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

    #[bench]
    fn easy_boards(b : &mut Bencher) {
        b.iter(|| 2+2);
    }
}