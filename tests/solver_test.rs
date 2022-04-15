#[cfg(test)]
mod tests {

    use connect4::{
        self,
        connect::{board::Board, solver::Solver},
    };
    use serde::Deserialize;
    use std::{
        error::Error,
        fmt::{self, Display},
        path::PathBuf,
    };

    #[derive(Debug, Deserialize, Clone, Copy)]
    struct Record {
        board: Board,
        score: i32,
    }
    impl fmt::Display for Record {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{} {}", self.board, self.score)
        }
    }
    fn test_from_file(path: PathBuf) -> Result<(), Box<dyn Error>> {
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_path(path);
        match rdr {
            Ok(mut result) => {
                for (i, value) in result.deserialize().enumerate() {
                    //if i > 100 {break;}
                    let mut solver: Solver = Solver::new();
                    let record: Record = value?;
                    let score = solver.solve(record.board);
                    assert_eq!(
                        score, record.score,
                        "The score should {} but scored {}, for test number {}",
                        record.score, score, i
                    );
                }
            }
            Err(_err) => {
                assert!(false);
            }
        }
        return Ok(());
    }

    #[test]
    fn end_easy_boards_test() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L3_R1");
        test_from_file(test_data).unwrap();
    }

    #[test]
    fn middle_easy_boards_test() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L2_R1");
        test_from_file(test_data).unwrap();
    }

    #[test]
    fn middle_medium_boards_test() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L2_R2");
        test_from_file(test_data).unwrap();
    }
    #[test]
    fn begin_easy_boards_test() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L1_R1");
        test_from_file(test_data).unwrap();
    }
    #[test]
    fn begin_medium_boards_test() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L1_R2");
        test_from_file(test_data).unwrap();
    }
    #[test]
    fn begin_hard_boards_test() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("tests/benchmark_positions/Test_L1_R3");
        test_from_file(test_data).unwrap();
    }
}
