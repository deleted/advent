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
    cells: Vec<Vec<char>>,
}

impl Pattern {
    fn new() -> Self {
        Self { cells: Vec::new() }
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

    fn flip_at(&mut self, (row, col): (usize, usize)) {
        self.cells[row][col] = match self.cells[row][col] {
            '.' => '#',
            '#' => '.',
            _ => panic!("unexpected char in cells"),
        }
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
        Pattern { cells: col_major }
    }

    fn find_reflections(&self) -> Vec<usize> {
        let mut reflections = Vec::new();
        let cells = &self.cells;
        for i in 1..cells.len() {
            let (mut left, mut right) = cells.split_at(i);
            let overlap = left.len().min(right.len());
            // trim to the same size
            let is_mirrored = (0..overlap).all(|i| left[left.len() - 1 - i] == right[i]);
            if is_mirrored {
                // dbg!(&left[left.len()-1-i], &right[i]);
                // dbg!(left, right);
                reflections.push(i);
            }
        }
        reflections
    }

    fn print(&self) {
        for row in &self.cells {
            println!("{}",row.iter().collect::<String>());
        }
    }

    fn find_smudge(&self) -> Option<(usize, usize, usize)> {
        let cells = &self.cells;
        for fold_point in 1..cells.len() {
            let (left, right) = cells.split_at(fold_point);
            let overlap = left.len().min(right.len());
            let distances: Vec<Vec<usize>> = (0..overlap)
                .map(|i| distance(&left[left.len() - 1 - i], &right[i]))
                .collect();
            let mut total_dist: usize = 0;
            let mut last_diff: (usize, usize, usize) = (0, 0, 0);
            for (i, dvec) in distances.iter().enumerate() {
                for (j, d) in dvec.iter().enumerate() {
                    total_dist += *d;
                    if *d == 1 {
                        last_diff = (fold_point + i, j, fold_point);
                    }
                }
            }
            dbg!(fold_point, total_dist);
            if total_dist == 1 {
                // smudge is at this fold
                return Some(last_diff);
            }
        }
        None
    }

}

fn distance(a: &Vec<char>, b: &Vec<char>) -> Vec<usize> {
    assert_eq!(a.len(), b.len());
    let mut diff: Vec<usize> = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        diff.push(match a[i] == b[i] {
            true => 0,
            false => 1,
        })
    }
    diff
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut pattern = Pattern::new();
    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            if !pattern.is_empty() {
                patterns.push(pattern);
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
    patterns
}

fn process_a(input: &str) -> i32 {
    let patterns = parse_input(input);
    let mut total = 0;
    for p in patterns {
        // let horizontal_reflections = p.find_reflections();
        // let veritical_reflections = p.transpose().find_reflections();
        p.transpose()
            .find_reflections()
            .iter()
            .for_each(|f| total += f);
        p.find_reflections().iter().for_each(|f| total += 100 * f);
    }
    total as i32
}

fn process_b(input: &str) -> i32 {
    let mut patterns = parse_input(input);
    let mut total = 0;
    for (pattern_idx, pattern) in patterns.iter_mut().enumerate() {
        dbg!(pattern_idx);
        let mut transposed = pattern.transpose();
        match transposed.find_smudge() {
            Some((col, row, fold_point)) => {
                println!("Smudge at {:?}", (col, row));
                transposed.flip_at((col, row));
                pattern.flip_at((row, col));
                total += fold_point;
            }
            None => {
                println!("No smudge in the transpose. Checking row picture.");
                match pattern.find_smudge() {
                    Some((row, col, fold_point)) => {
                        println!("Smudge at {:?}", (row, col));
                        let before = pattern.cells[row][col];
                        pattern.flip_at((row, col));
                        transposed.flip_at((col, row));
                        assert!(before != pattern.cells[row][col]);
                        assert_eq!(pattern.cells[row][col], transposed.cells[col][row]);
                        total += 100 * fold_point;
                    }
                    None => panic!("No smudge found"),
                }
            }
        }
        ///  Ugh! it was super unclear in the description that they were looking for only 
        /// the new reflections. Not the total of all reflections (as in part A) after fixing the smudge.
        // pattern.print();
        // transposed
        //     .find_reflections()
        //     .iter()
        //     .for_each(|f| total += f);
        // pattern
        //     .find_reflections()
        //     .iter()
        //     .for_each(|f| total += 100 * f);
    }
    total as i32
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
        let expected_output = 400;
        assert_eq!(process_b(input), expected_output);
    }
}
