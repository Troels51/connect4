
use std::{error::Error, path::{PathBuf}, fmt::{Display, self}, time::Duration};
use criterion::{Criterion, BenchmarkId, criterion_main, criterion_group};
use connect4::{self, connect::{board::Board, solver::Solver}};
use serde::{Deserialize};



#[derive(Debug, Deserialize, Clone, Copy)]
struct Record {
    board : Board,
    score : i32,
}

fn bench_from_file(c: &mut Criterion, path: PathBuf) -> Result<(), Box<dyn Error >>{
    let mut group = c.benchmark_group(path.to_str().expect("Expect path to be string like"));
    group.sample_size(10);
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path(path);
    match rdr {
        Ok(mut result) => {
            for (i, value)in result.deserialize().enumerate() {
                if i > 5 {
                    break;
                }
                let mut solver : Solver = Solver::new();
                let record : Record = value?;
                group.bench_with_input(BenchmarkId::from_parameter(i), &record,
                |b, val| {
                        b.iter(|| {
                            let (score, plays) = solver.solve(val.board);
                            assert_eq!(score, record.score, "The score should {} but scored {}, for this board {}", record.score, score, record.board);
                        });
                    }
                );
            }
        }
        Err(_err) => {assert!(false);}
    }
    return Ok(());
}

fn end_easy_boards_bench(c: &mut Criterion){
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/benchmark_positions/Test_L3_R1");
    bench_from_file(c, test_data).unwrap();
}
fn middle_easy_boards_bench(c: &mut Criterion){
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/benchmark_positions/Test_L2_R1");
    bench_from_file(c,test_data).unwrap();
}

fn middle_medium_boards_bench(c: &mut Criterion){
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/benchmark_positions/Test_L2_R2");
    bench_from_file(c,test_data).unwrap();
}
fn begin_easy_boards_bench(c: &mut Criterion){
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/benchmark_positions/Test_L1_R1");
    bench_from_file(c,test_data).unwrap();
}
fn begin_medium_boards_bench(c: &mut Criterion){
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/benchmark_positions/Test_L1_R2");
    bench_from_file(c,test_data).unwrap();
}
fn begin_hard_boards_bench(c: &mut Criterion){
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("tests/benchmark_positions/Test_L1_R3");
    bench_from_file(c,test_data).unwrap();
}

criterion_group!(benches,
    end_easy_boards_bench,
    middle_easy_boards_bench,
    middle_medium_boards_bench,
    begin_easy_boards_bench,
    begin_medium_boards_bench,
    //begin_hard_boards_bench
);

criterion_main!(benches
);
