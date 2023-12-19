use std::collections::VecDeque;
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
    let seqs = input
        .lines()
        .map(|line| Sequence::init(line))
        .collect::<Vec<Sequence>>();
    let mut results = Vec::new();
    for mut seq in seqs {
        seq.gen_deltas();
        results.push(seq.extrapolate());
    }
    results.iter().fold(0, |acc, val| acc + *val)
}

fn process_b(input: &str) -> i32 {
    let seqs = input
        .lines()
        .map(|line| Sequence::init(line))
        .collect::<Vec<Sequence>>();
    let mut results = Vec::new();
    for mut seq in seqs {
        seq.gen_deltas();
        results.push(seq.extrapolate_front());
    }
    results.iter().fold(0, |acc, val| acc + *val)
}

struct Sequence {
    vecs: Vec<VecDeque<i32>>,
}

impl Sequence {
    fn init(line: &str) -> Self {
        let seq: VecDeque<i32> = line
            .trim()
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().expect("not a number"))
            .collect();
        Self { vecs: vec![seq] }
    }

    fn gen_deltas(&mut self) {
        let mut vec_idx = 0;
        loop {
            let current_vec = self.vecs.get(vec_idx).unwrap();
            let mut deltas = VecDeque::<i32>::new();
            for i in 1..current_vec.len() {
                deltas.push_back(current_vec[i] - current_vec[i - 1]);
            }
            assert!(deltas.len() == current_vec.len() - 1);
            self.vecs.push(deltas);
            vec_idx += 1;
            if self.vecs[vec_idx].iter().all(|val| *val == 0) {
                break;
            }
        }
        // println!("Stopped looping after computing {} delta series", vec_idx);
    }

    fn extrapolate(&mut self) -> i32 {
        // let zeros = &mut self.vecs.last().unwrap();
        // assert!(zeros.iter().all(|v| *v == 0));
        // zeros.push(0);
        let n_vecs = self.vecs.len();
        self.vecs[n_vecs - 1].push_back(0);
        for vec_idx in (n_vecs - 2)..0 {
            let last_delta = self.vecs[vec_idx + 1].back().unwrap().clone();
            let this_vec = &mut self.vecs[vec_idx];
            this_vec.push_back(this_vec.back().unwrap() + last_delta);
        }

        let sum = self
            .vecs
            .iter()
            .fold(0, |acc, val| acc + val.back().unwrap());
        return sum;
    }

    fn extrapolate_front(&mut self) -> i32 {
        let n_vecs = self.vecs.len();
        self.vecs[n_vecs - 1].push_front(0);
        let mut itercount = 0;
        for vec_idx in (0..(n_vecs - 2)).rev() {
            itercount += 1;
            let last_delta = self.vecs[vec_idx + 1].front().unwrap().clone();
            let this_vec = &mut self.vecs[vec_idx];
            this_vec.push_front(this_vec.front().unwrap() - last_delta);
            // print!("{:?}", this_vec);
        }

        *self.vecs[0].front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn test_a() {
        let expected_output = 114;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let inputb = "10 13 16 21 30 45";
        let expected_output = 2;
        assert_eq!(process_b(input), expected_output);
    }
}
