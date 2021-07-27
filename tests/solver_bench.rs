#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use std::{error::Error, path::{PathBuf}};
    use test::Bencher;
    use connect4::{self, connect::{board::Board, solver::Solver}};
    use serde::{Deserialize};



    #[derive(Debug, Deserialize)]
    struct Record {
        board : Board,
        score : i32,
    }
    fn test_from_file(path: PathBuf) -> Result<(), Box<dyn Error >>{
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_path(path);
        match rdr {
            Ok(mut result) => {
                for value in result.deserialize() {
                    let mut solver : Solver = Solver::new();
                    let record : Record = value?;
                    let (score, plays) = solver.solve(record.board);
                    assert_eq!(score, record.score, "The score should {} but scored {}, for this board {}", record.score, score, record.board);
                }
            }
            Err(_err) => {assert!(false);}
        }
        return Ok(());
    }
    #[test]
    fn end_easy_boards_test(){
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L3_R1");
        test_from_file(test_data).unwrap();
    }
    #[bench]
    fn end_easy_boards_bench(b : &mut Bencher) {
        b.iter(|| end_easy_boards_test());
    }

    #[test]
    fn middle_easy_boards_test(){
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L2_R1");
        test_from_file(test_data).unwrap();
    }
    #[bench]
    fn middle_easy_boards_bench(b : &mut Bencher) {
        b.iter(|| middle_easy_boards_test());
    }
    #[test]
    fn middle_medium_boards_test(){
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L2_R2");
        test_from_file(test_data).unwrap();
    }
    #[bench]
    fn middle_medium_boards_bench(b : &mut Bencher) {
        b.iter(|| middle_medium_boards_test());
    }
    #[test]
    fn begin_easy_boards_test(){
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L1_R1");
        test_from_file(test_data).unwrap();
    }
    #[bench]
    fn begin_easy_boards_bench(b : &mut Bencher) {
        b.iter(|| begin_easy_boards_test());
    }
    #[test]
    fn begin_medium_boards_test(){
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L1_R2");
        test_from_file(test_data).unwrap();
    }
    #[bench]
    fn begin_medium_boards_bench(b : &mut Bencher) {
        b.iter(|| begin_medium_boards_test());
    }
    #[test]
    fn begin_hard_boards_test(){
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L1_R3");
        test_from_file(test_data).unwrap();
    }
    #[bench]
    fn begin_hard_boards_bench(b : &mut Bencher) {
        b.iter(|| begin_hard_boards_test());
    }
}