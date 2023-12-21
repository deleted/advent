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

#[derive(Debug)]
struct Pattern {
    cells: Vec<Vec<char>>
}

impl Pattern {
    fn new() -> Self {
        Self{
            cells: Vec::new()
        }
    }

    fn add_row(&mut self, line: &str) {
        let mut row = Vec::new();
        for char in line.chars() {
            assert!("#.".contains(char));
            row.push(char);
        }
        self.cells.push(row);
    }

    fn is_empty(&self) -> bool {
        return self.cells.is_empty();
    }

    fn transpose(&self) -> Pattern {
        let n_cols = self.cells[0].len();
        let mut col_major = Vec::new();
        (0..n_cols).for_each(|_| col_major.push(Vec::new()));
        for row in self.cells.iter() {
            for (i, c) in row.iter().enumerate() {
                col_major[i].push(*c);
            }
        }
        Pattern{cells: col_major}
    }

    fn find_reflections(&self) -> Vec<usize> {
        let mut folds = Vec::new();
        let cells = &self.cells;
        for i in 1..cells.len() {
            let (mut left, mut right) = cells.split_at(i);
            let overlap = left.len().min(right.len());
            // trim to the same size
            let is_mirrored = (0..overlap).all(|i| left[left.len()-1-i] == right[i]);
            if is_mirrored {
                // dbg!(&left[left.len()-1-i], &right[i]);
                // dbg!(left, right);
                folds.push(i);
            }
        }
        folds
    }
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut pattern = Pattern::new();
    let mut pushes = 0;
    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            if !pattern.is_empty() {
                patterns.push(pattern);
                pushes += 1;
                pattern = Pattern::new();
            }
            continue;
        } else {
            pattern.add_row(line);
        }
    }
    if !pattern.is_empty() {
        patterns.push(pattern);
    }
    dbg!(pushes);
    patterns
}

fn process_a(input: &str) -> i32 {
    let patterns = parse_input(input);
    let mut total = 0;
    for p in patterns {
        // let horizontal_reflections = p.find_reflectoins();
        // let veritical_reflections = p.transpose().find_reflectoins();
        // dbg!(vertical_folds, horizontal_folds);
        p.transpose().find_reflections().iter().for_each(|f| total += f);
        p.find_reflections().iter().for_each(|f| total += 100*f);
    }
    total as i32
}

fn process_b(_: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#";

    #[test]
    fn test_a() {
        let expected_output = 405;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let expected_output = 0;
        assert_eq!(process_b(input), expected_output);
    }
}
