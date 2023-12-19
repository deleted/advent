use anyhow::{anyhow, Result};
use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg()]
    input_file: std::path::PathBuf,
}

fn main() {
    let cli = Args::parse();
    let number_pattern = Regex::new(r"([0-9]+)").unwrap();

    let mut symbol_map = SymbolMap::new();
    let mut numbers: Vec<Number> = vec![];
    let mut stars = Vec::new();
    let mut total = 0;

    let file = File::open(cli.input_file).unwrap();
    let reader = BufReader::new(file);
    let mut rowcount = 0;
    let mut colcount = 0;
    for (row_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        rowcount += 1;
        colcount = 0;
        symbol_map.new_row();
        for char in line.chars() {
            symbol_map
                .push(!(char.is_numeric() || char == '.'))
                .unwrap();
            if char == '*' {
                stars.push(Point {
                    x: colcount,
                    y: row_idx as i32,
                });
            }
            colcount += 1;
        }
        for m in number_pattern.find_iter(&line) {
            numbers.push(Number {
                value: m.as_str().parse().unwrap(),
                position: Region {
                    line: row_idx as i32,
                    start_idx: m.start() as i32,
                    end_idx: m.end() as i32,
                },
            })
        }
    }

    let mut number_map = NumberMap::new(rowcount, colcount);

    for num in &numbers {
        number_map.add_number(num).unwrap();
        let mut important = false;
        if symbol_map.is_region_adjacent_to_symbol(&num.position) {
            total += num.value;
            important = true;
        }
        // println!("{}: {}", num.to_string(), important)
    }
    println! {"Total is {}", total}

    let mut total_ratios = 0;
    for starloc in stars {
        println!("Considering star at {}", starloc);
        let numbers = number_map.get_adjacent_numbers(starloc.y, starloc.x);
        println!(
            "Gears: {:?}",
            numbers.iter().map(|n| n.value).collect::<Vec<i32>>()
        );
        if numbers.len() == 2 {
            total_ratios += numbers
                .iter()
                .map(|n| n.value)
                .reduce(|a, n| a * n)
                .unwrap();
        }
    }
    println!("Total ratios: {}", total_ratios);
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Region {
    line: i32,
    start_idx: i32,
    end_idx: i32,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Number {
    value: i32,
    position: Region,
}

impl Number {
    fn to_string(&self) -> String {
        format!(
            "{} [{} {}-{}]",
            self.value, self.position.line, self.position.start_idx, self.position.end_idx
        )
    }
}

impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.position.line.hash(state);
        self.position.start_idx.hash(state);
        self.position.end_idx.hash(state);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}

struct SymbolMap(Vec<Vec<bool>>);

impl SymbolMap {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn new_row(&mut self) {
        self.0.push(Vec::new())
    }

    fn push(&mut self, is_symbol: bool) -> Result<()> {
        let n_rows = self.0.len();
        let row = self.0.get_mut(n_rows - 1).ok_or(anyhow!("no rows"))?;
        row.push(is_symbol);
        Ok(())
    }

    fn is_symbol(&self, row: i32, col: i32) -> bool {
        if row < 0 || col < 0 {
            // out of bounds
            return false;
        };
        let row_idx = row as usize;
        let col_idx = col as usize;
        match self.0.get(row_idx) {
            None => return false,
            Some(row) => match row.get(col_idx) {
                None => return false,
                Some(val) => {
                    return *val;
                }
            },
        }
    }

    fn is_region_adjacent_to_symbol(&self, region: &Region) -> bool {
        let mut cursor = Point {
            x: region.start_idx,
            y: region.line,
        };
        // println!("Cursor start: ({}, {})", cursor.y, cursor.x);
        while cursor.x < region.end_idx {
            // scan around each character for a symbol
            for row_offset in -1..2 {
                assert!(row_offset >= -1 && row_offset <= 1);
                let y = cursor.y as i32 + row_offset;
                for col_offset in -1..2 {
                    assert!(col_offset >= -1 && col_offset <= 1);
                    let x = cursor.x as i32 + col_offset;
                    // println!("({}, {}): {}", y, x, self.is_symbol(y, x));
                    match self.is_symbol(y, x) {
                        true => return true,
                        false => continue,
                    }
                }
            }

            cursor.x += 1;
        }
        return false;
    }

    fn as_str(&self) -> String {
        let mut out: Vec<u8> = Vec::new();
        for row in &self.0 {
            for b in row {
                let c = match *b {
                    true => b't',
                    false => b'.',
                };
                out.push(c.try_into().unwrap());
            }
            out.push(b'\n')
        }
        String::from_utf8(out).unwrap()
    }
}

struct NumberMap<'a>(Vec<Vec<Option<&'a Number>>>);

impl<'a> NumberMap<'a> {
    fn new(rows: i32, cols: i32) -> Self {
        let mut map = Self(Vec::new());
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(None);
            }
            map.0.push(row);
        }
        map
    }

    fn add_number(&mut self, num: &'a Number) -> Result<()> {
        let row = self
            .0
            .get_mut(num.position.line as usize)
            .ok_or(anyhow!("out of bounds"))?;
        let mut count = 0;
        for i in num.position.start_idx..num.position.end_idx {
            if row[i as usize].is_some() {
                return Err(anyhow!(
                    "trying to overwrite a filled region in the number map"
                ));
            }
            row[i as usize] = Some(num);
            count += 1;
        }
        Ok(())
    }

    fn get_adjacent_numbers(&self, row: i32, col: i32) -> Vec<Number> {
        let mut numbers = HashSet::new();
        for row_offset in -1..2 {
            let y = row + row_offset;
            if y < 0 || y > (self.0.len() as i32 - 1) {
                // edge of board. ignore
                continue;
            }
            let maprow = self.0.get(y as usize).unwrap();
            for col_offset in -1..2 {
                let x = col + col_offset;
                if x < 0 {
                    continue;
                }
                match maprow.get(x as usize).unwrap() {
                    None => continue,
                    Some(num) => numbers.insert(**num),
                };
            }
        }

        let numbers_vec = numbers.iter().map(|n| n.clone()).collect::<Vec<Number>>();
        numbers_vec
    }

    fn as_str(&self) -> String {
        let mut out: Vec<u8> = Vec::new();
        for row in &self.0 {
            for b in row {
                let c = match *b {
                    None => '.',
                    Some(n) => n.value.to_string().chars().next().unwrap(),
                };
                out.push(c.try_into().unwrap());
            }
            out.push(b'\n')
        }
        String::from_utf8(out).unwrap()
    }
}
