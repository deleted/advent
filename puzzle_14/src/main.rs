use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time;
use std::{fs::File, io::Read};

use clap::{arg, command, Parser};
use pathfinding::directed::cycle_detection::floyd;

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
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    Round,
    Square,
    Empty,
}
use Item::*;

#[derive(Debug, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Hash, Clone, PartialEq)]
struct Platform {
    grid: Vec<Vec<Item>>,
}

impl Platform {
    fn load(input: &str) -> Self {
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.trim().chars() {
                row.push(match char {
                    'O' => Round,
                    '#' => Square,
                    '.' => Empty,
                    _ => panic!("Unexpected char '{}'", char),
                });
            }
            grid.push(row);
        }
        Self { grid }
    }

    fn concrete_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        return hasher.finish();
    }

    fn in_bounds(&self, row: i32, col: i32) -> bool {
        let row_count = self.grid.len() as i32;
        let col_count = match row_count > 0 {
            false => 0,
            true => self.grid[0].len() as i32,
        };
        return row >= 0 && row < row_count && col >= 0 && col < col_count;
    }

    fn get(&self, coord: Coord) -> Item {
        self.grid[coord.row][coord.col]
    }

    fn put(&mut self, coord: Coord, val: Item) {
        self.grid[coord.row][coord.col] = val;
    }

    fn look(&self, from: Coord, dir: Direction) -> Option<Item> {
        let mut row = from.row as i32;
        let mut col = from.col as i32;
        match dir {
            Direction::North => row -= 1,
            Direction::South => row += 1,
            Direction::West => col -= 1,
            Direction::East => col += 1,
        }
        return match self.in_bounds(row, col) {
            true => Some(self.get(Coord {
                row: row as usize,
                col: col as usize,
            })),
            false => None,
        };
    }

    fn swap_at(&mut self, src: Coord, dir: Direction) {
        let dst = match dir {
            Direction::North => Coord {
                row: src.row - 1,
                col: src.col,
            },
            Direction::South => Coord {
                row: src.row + 1,
                col: src.col,
            },
            Direction::West => Coord {
                row: src.row,
                col: src.col - 1,
            },
            Direction::East => Coord {
                row: src.row,
                col: src.col + 1,
            },
        };
        assert!(self.get(src) == Round);
        assert!(self.get(dst) == Empty);
        self.put(dst, Round);
        self.put(src, Empty);
    }

    fn tilt(&mut self, dir: Direction) {
        loop {
            let mut swaps = Vec::new();
            for (r, row) in self.grid.iter().enumerate() {
                for (c, item) in row.iter().enumerate() {
                    if *item == Item::Round {
                        let coord = Coord { row: r, col: c };
                        if self.look(coord, dir).is_some_and(|i| i == Item::Empty) {
                            swaps.push((coord, dir));
                        }
                    }
                }
            }
            for (coord, dir) in &swaps {
                self.swap_at(*coord, *dir);
            }

            if swaps.len() == 0 {
                break;
            }
        }
    }

    fn total_load(&self) -> i32 {
        let mut total = 0;
        let n_rows = self.grid.len();
        for (row_idx, row) in self.grid.iter().enumerate() {
            for item in row {
                if *item == Item::Round {
                    total += n_rows - row_idx;
                }
            }
        }
        total as i32
    }
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for row in &self.grid {
            for col in row {
                s.push(match col {
                    Empty => '.',
                    Round => 'O',
                    Square => '#',
                })
            }
            s.push('\n')
        }
        s
    }
}

fn process_a(input: &str) -> i32 {
    let mut platform = Platform::load(input);
    println!(
        "loaded platform: ({},{})",
        platform.grid.len(),
        platform.grid[0].len()
    );
    loop {
        let mut changes = 0;
        let mut swap_locs = Vec::new();
        for (row_idx, row) in platform.grid.iter().enumerate() {
            for (col_idx, _val) in row.iter().enumerate() {
                let this_loc = Coord {
                    row: row_idx,
                    col: col_idx,
                };
                if this_loc.row > 0 {
                    let dest_loc = Coord::new(this_loc.row - 1, this_loc.col);
                    if platform.get(this_loc) == Item::Round && platform.get(dest_loc) == Empty {
                        // dbg!("push", &this_loc);
                        swap_locs.push(this_loc);
                        changes += 1;
                    }
                }
            }
        }
        dbg!(changes, swap_locs.len());
        for loc in swap_locs {
            // dbg!(&loc);
            platform.swap_at(loc, Direction::North);
        }
        // println!("{}", platform.to_string());
        if changes == 0 {
            break;
        }
    }
    platform.total_load()
}

fn process_b(input: &str) -> i32 {
    let platform = Platform::load(input);

    fn rotate(input: Platform) -> Platform {
        let mut output = input.clone();
        // println!("{}", output.to_string());
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            output.tilt(dir);
            // print!("{}", output.to_string());
        }
        return output;
    }

    // dbg!(platform.concrete_hash());
    // println!("{}", platform.to_string());
    let (cycle_size, mut platform, start_idx) = floyd(platform, |p| {
        let platform = rotate(p);
        // dbg!(platform.concrete_hash());
        // println!("{}", platform.to_string());
        return platform;
    });
    dbg!(cycle_size, start_idx);
    let total_rotates = 1000000000;
    let end_cycle_point = (total_rotates - start_idx) % cycle_size;
    dbg!(end_cycle_point);
    let mut loads = Vec::new();
    loads.push(platform.total_load());
    for _ in 0..end_cycle_point {
        platform = rotate(platform);
        loads.push(platform.total_load());
    }
    dbg!(loads);
    platform.total_load()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";

    #[test]
    fn test_a() {
        let expected_output = 136;
        assert_eq!(process_a(INPUT), expected_output);
    }

    #[test]
    fn test_b() {
        let expected_output = 64;
        assert_eq!(process_b(INPUT), expected_output);
    }

    #[test]
    fn test_swap() {
        let mut platform = Platform {
            grid: { vec![vec![Item::Empty], vec![Item::Round]] },
        };
        platform.swap_at(Coord { row: 1, col: 0 }, Direction::North);
        assert_eq!(platform.get(Coord { row: 0, col: 0 }), Item::Round);
        assert_eq!(platform.get(Coord { row: 1, col: 0 }), Item::Empty);
    }
}
