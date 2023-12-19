use std::time;
use std::{fs::File, io::Read};

use clap::{arg, command, Parser};
use itertools::Itertools;

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

#[derive(Debug, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    id: usize,
    position: Coord,
}

impl Galaxy {
    fn move_right(&mut self, amount: i32) {
        self.position.col += amount as usize;
    }

    fn move_down(&mut self, amount: i32) {
        self.position.row += amount as usize;
    }
}

struct Size {
    rows: usize,
    cols: usize,
}

fn parse_input(input: &str) -> (Vec<Galaxy>, Size) {
    let mut galaxies = Vec::new();
    let mut idx = 0;
    let mut rows = 0;
    let mut cols = 0;
    for (row, line) in input.lines().enumerate() {
        rows = rows.max(row);
        for (col, char) in line.chars().enumerate() {
            cols = cols.max(col);
            match char {
                '#' => {
                    idx += 1;
                    galaxies.push(Galaxy {
                        id: idx,
                        position: Coord { row, col },
                    })
                }
                _ => continue,
            }
        }
    }
    let size = Size {
        rows: rows + 1,
        cols: cols + 1,
    };
    (galaxies, size)
}

fn expand_galaxies(galaxies: &mut Vec<Galaxy>, size: &Size, amount: i32) -> Vec<Galaxy> {
    let mut emptyrows = vec![true; size.rows];
    let mut emptycols = vec![true; size.cols];

    let mut expanded_galaxies = Vec::new();

    for gxy in &mut *galaxies {
        emptycols[gxy.position.col] = false;
        emptyrows[gxy.position.row] = false;
        expanded_galaxies.push(gxy.clone())
    }

    let mut col: usize = emptycols.len();
    while col > 0 {
        col -= 1;
        match emptycols[col] {
            false => (),
            true => {
                for (i, original_gxy) in &mut galaxies.iter().enumerate() {
                    if original_gxy.position.col > col {
                        let ng = expanded_galaxies.get_mut(i).unwrap();
                        assert_eq!(original_gxy.id, ng.id);
                        ng.move_right(amount);
                    }
                }
            }
        }
    }

    let mut row: usize = emptycols.len();
    while row > 0 {
        row -= 1;
        match emptyrows[row] {
            false => (),
            true => {
                for (i, gal) in galaxies.iter().enumerate() {
                    if gal.position.row > row {
                        let ng = expanded_galaxies.get_mut(i).unwrap();
                        ng.move_down(amount);
                    }
                }
            }
        }
    }
    expanded_galaxies
}

fn compute_distance(galaxies: &Vec<Galaxy>) -> i32 {
    let mut total = 0;
    for combo in galaxies.iter().combinations(2) {
        let a = combo[0];
        let b = combo[1];
        let distance =
            a.position.row.abs_diff(b.position.row) + a.position.col.abs_diff(b.position.col);
        total += distance;
    }
    total as i32
}

///////////
///

fn process_a(input: &str) -> i32 {
    process_n(input, 2)
}

fn process_b(input: &str) -> i32 {
    process_n(input, 1000000)
}

fn process_n(input: &str, n: i32) -> i32 {
    let (mut galaxies, size) = parse_input(input);
    let expanded = expand_galaxies(&mut galaxies, &size, n - 1);
    compute_distance(&expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_a() {
        let expected_output = 374;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        assert_eq!(process_n(input, 10), 1030);
        assert_eq!(process_n(input, 100), 8410);
        // let expected_output = 8410;
        // assert_eq!(process_b(input), expected_output);
    }
}
