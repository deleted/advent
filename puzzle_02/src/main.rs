use std::cmp::max;
use std::io::BufRead;
use std::str::FromStr;
use std::{fs::File, io::BufReader};

use anyhow::{anyhow, Result};
use clap::Parser;
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum Color {
    #[strum(ascii_case_insensitive)]
    Red,
    #[strum(ascii_case_insensitive)]
    Green,
    #[strum(ascii_case_insensitive)]
    Blue,
}

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg()]
    input_file: std::path::PathBuf,
}

fn main() {
    let cli = Args::parse();

    let bag = RgbCount(12, 13, 14);

    let file = File::open(cli.input_file).unwrap();
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut total = 0;
    let mut total_power = 0;
    for line in reader.lines() {
        count += 1;
        let game = parse_line(&line.unwrap()).unwrap();
        if game.cubes.iter().all(|set| bag.can_contain(set)) {
            total += game.idx;
        }
        total_power += find_minimum_cubes(&game).power();
    }

    println!("Read {} lines", count);
    println!("Total of fitting indexes: {}", total);
    println!("Total power: {}", total_power);
}

#[derive(Copy, Clone)]
struct RgbCount(i32, i32, i32);

impl RgbCount {
    fn can_contain(&self, other: &RgbCount) -> bool {
        other.0 <= self.0 && other.1 <= self.1 && other.2 <= self.2
    }

    fn power(&self) -> i32 {
        self.0 * self.1 * self.2
    }
}

struct Game {
    idx: i32,
    cubes: Vec<RgbCount>,
}

fn parse_line(line: &str) -> Result<Game> {
    let colon_idx = line.find(":").ok_or(anyhow!("no colon"))?;
    let mut cubes: Vec<RgbCount> = Vec::new();
    let idx_str = line[..colon_idx].split(" ").last().unwrap();
    let idx = idx_str.parse::<i32>().or(Err(anyhow!(
        "Could not parse an integer from:\"{}\"",
        idx_str
    )))?;
    let draws = line[colon_idx + 1..].split(";");
    for draw in draws {
        let mut counts = RgbCount(0, 0, 0);
        for c in draw.split(",") {
            let mut sw = c.split_whitespace();
            let number_str = sw.next().unwrap();
            let number = number_str
                .parse::<i32>()
                .or(Err(anyhow!("not an int: \"{}\"", number_str)))?;
            let color_str = sw.next().unwrap();
            let color =
                Color::from_str(color_str).or(Err(anyhow!("\"{}\" not a color", color_str)))?;
            match color {
                Color::Red => counts.0 += number,
                Color::Green => counts.1 += number,
                Color::Blue => counts.2 += number,
            }
            cubes.push(counts);
        }
    }
    Ok(Game { idx, cubes })
}

fn find_minimum_cubes(game: &Game) -> RgbCount {
    let mut minset = RgbCount(0, 0, 0);
    for draw in &game.cubes {
        minset.0 = max(minset.0, draw.0);
        minset.1 = max(minset.1, draw.1);
        minset.2 = max(minset.2, draw.2);
    }
    minset
}
