#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use std::{error::Error, fs::File, io::{self, BufRead}, path::{Path, PathBuf}};
    use test::Bencher;
    use connect4;
    use csv::Reader;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Record {
        board : String,
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
                    println!("{:?}", record);
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
        test_from_file(test_data);
    }
    #[bench]
    fn easy_boards(b : &mut Bencher) {
        b.iter(|| 2+2);
    }
}