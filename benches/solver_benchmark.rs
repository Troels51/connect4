#![allow(unused)]
fn main() {
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use connect4::connect;

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
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
}
