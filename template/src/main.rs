use std::sync::Arc;
use std::time;
use std::{fs::File, io::Read};

use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg()]
    input_file: std::path::PathBuf,
}

fn main() {
    let cli = Args::parse();
    let mut input = String::new();
    File::open(cli.input_file)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let t0 = time::Instant::now();
    let result = process_a(&input);
    let dur = time::Instant::now() - t0;
    println!("Result A: {result} in {:?}", dur);

    let t1 = time::Instant::now();
    let result_b = process_b(&input);
    let dur = time::Instant::now() - t1;
    println!("Result B: {result_b} in {:?}", dur);
}

fn process_a(input: &str) -> i32 {
    0
}

fn process_b(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "";

    #[test]
    fn test_a() {
        let expected_output = 0;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let expected_output = 0;
        assert_eq!(process_b(input), expected_output);
    }
}
