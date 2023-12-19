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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}
use Dir::*;

impl Dir {
    fn invert(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

fn dirs_for_char(c: &char) -> Option<[Dir; 2]> {
    match c {
        '|' => Some([North, South]),
        '-' => Some([West, East]),
        'L' => Some([North, East]),
        'J' => Some([North, West]),
        '7' => Some([West, South]),
        'F' => Some([South, East]),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn step(&self, dir: &Dir) -> Coord {
        let mut next = self.clone();
        match dir {
            North => next.row -= 1,
            South => next.row += 1,
            East => next.col += 1,
            West => next.col -= 1,
        }
        next
    }
}

struct PipeGrid {
    cells: Vec<Vec<char>>,
    start_pos: Coord,
}

impl PipeGrid {
    fn from_input(input: &str) -> Self {
        let mut start_point = Coord { row: 0, col: 0 };
        let mut cells = Vec::new();
        for (r, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (c, val) in line.trim().chars().enumerate() {
                row.push(val);
                if val == 'S' {
                    start_point = Coord { row: r, col: c };
                }
            }
            cells.push(row);
        }
        Self {
            cells,
            start_pos: start_point,
        }
    }

    fn char_at(&self, pos: &Coord) -> char {
        self.cells[pos.row][pos.col]
    }
}

struct Traversal {
    grid: PipeGrid,
    start_point: Coord,
    cursor: Coord,
    came_from: Option<Dir>,
    step_count: i32,
    path: Vec<Coord>,
}

impl Traversal {
    fn for_grid(grid: PipeGrid) -> Self {
        Self {
            start_point: grid.start_pos,
            cursor: grid.start_pos,
            step_count: 0,
            came_from: None,
            path: vec![grid.start_pos],
            grid,
        }
    }

    fn step(&self, dir: &Dir) -> Option<Coord> {
        let mut next = self.cursor.clone();
        let n_rows = self.grid.cells.len();
        let n_cols = self.grid.cells[0].len();
        match dir {
            North => {
                if next.row > 0 {
                    next.row -= 1;
                } else {
                    return None;
                }
            }
            South => {
                if next.row < (n_rows - 1) {
                    next.row += 1;
                } else {
                    return None;
                }
            }
            East => {
                if next.col < (n_cols - 1) {
                    next.col += 1
                } else {
                    return None;
                }
            }
            West => {
                if next.col > 0 {
                    next.col -= 1
                } else {
                    return None;
                }
            }
        }
        Some(next)
    }

    fn scan_next_step(&self) -> Dir {
        let dirs: Vec<Dir> = match dirs_for_char(&self.grid.char_at(&self.cursor)) {
            Some(dirs) => dirs.iter().map(|d| d.clone()).collect(),
            None => [North, East, South, West]
                .iter()
                .map(|d| d.clone())
                .collect(),
        };

        for dir in dirs {
            match self.step(&dir) {
                None => continue,
                Some(there) => {
                    let entries = dirs_for_char(&self.grid.char_at(&there));
                    match entries {
                        None => match self.grid.char_at(&there) {
                            'S' => {
                                if self.came_from.unwrap() != dir {
                                    return dir;
                                } else {
                                    continue;
                                }
                            }
                            _ => continue,
                        },
                        Some(entries) => {
                            if self.came_from.is_some() && dir == *self.came_from.as_ref().unwrap()
                            {
                                continue;
                            }

                            if entries.iter().any(|d| *d == dir.invert()) {
                                return dir;
                            }
                        }
                    }
                }
            };
        }
        panic!("No path from {:?}", self.cursor);
    }

    fn traverse(&mut self) {
        let mut count = 0;
        loop {
            let next_move = self.scan_next_step();
            let next_pos = self.cursor.step(&next_move);
            // println!(
            //     "{}: {:?} to {}",
            //     count,
            //     &next_move,
            //     self.grid.char_at(&next_pos)
            // );
            self.cursor = next_pos;
            self.path.push(next_pos);
            self.came_from = Some(next_move.invert());
            if self.cursor == self.start_point {
                println!("Found circuit in {} steps", self.path.len());
                break;
            }
            count += 1;
        }
    }
}

fn process_a(input: &str) -> i32 {
    let grid = PipeGrid::from_input(input);
    let mut traversal = Traversal::for_grid(grid);
    traversal.traverse();
    let path_len = traversal.path.len();
    (path_len / 2) as i32
}

fn process_b(input: &str) -> i32 {
    let grid = PipeGrid::from_input(input);
    let mut traversal = Traversal::for_grid(grid);
    traversal.traverse();
    let path_len = traversal.path.len();

    // What follows is a combination of
    // The shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
    // and
    // Pick's Theorem: https://en.wikipedia.org/wiki/Pick%27s_theorem
    let mut sum: i32 = 0;
    for i in 0..path_len - 1 {
        let this = traversal.path.get(i).unwrap();
        let next = traversal.path.get(i + 1).unwrap();
        sum += (this.row as i32 + next.row as i32) * (this.col as i32 - next.col as i32);
    }
    let area = (sum / 2).abs();
    let interior_points = area + 1 - (path_len as i32 / 2);
    interior_points
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";

    static input_b: &str = "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_a() {
        let expected_output = 8;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let expected_output = 10;
        assert_eq!(process_b(input_b), expected_output);
    }
}
