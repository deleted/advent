use core::num;
use std::collections::BTreeMap;
use std::time;
use std::{collections::HashSet, fs::File, future::IntoFuture, io::Read};

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
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn get_win_ways_count(&self) -> i64 {
        let mut count = 0;
        for i in 0..self.time {
            let dist = i * (self.time - i);
            if dist > self.distance {
                count += 1
            }
        }
        count
    }

    fn distance_for_press(&self, sec: i64) -> i64 {
        sec * (self.time - sec)
    }

    fn get_ways_to_win_faster(&self) -> i64 {
        let mut start = 0;
        while start < self.time {
            if self.distance_for_press(start) > self.distance {
                break;
            }
            start += 1;
        }
        let mut end = self.time + 1;
        while end > 0 {
            if self.distance_for_press(end) > self.distance {
                break;
            }
            end -= 1;
        }
        end - start + 1
    }
}

fn process_a(input: &str) -> i64 {
    let mut lines = input.lines();
    let timeline = lines.next().unwrap();
    assert!(timeline.contains("Time:"));
    let times = timeline
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|tok| !tok.is_empty())
        .map(|tok| tok.parse::<i64>().expect("not a number"))
        .collect::<Vec<i64>>();
    let distance_line = lines.next().unwrap();
    assert!(distance_line.contains("Distance:"));
    let distances = distance_line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|tok| !tok.is_empty())
        .map(|tok| tok.parse::<i64>().expect("not a number"))
        .collect::<Vec<i64>>();

    let races = std::iter::zip(times, distances).map(|(time, distance)| Race { time, distance });
    let result = races
        .map(|r| r.get_win_ways_count())
        .reduce(|acc, c| acc * c)
        .unwrap();
    result
}

fn process_b(input: &str) -> i64 {
    let mut lines = input.lines();
    let timeline = lines.next().unwrap();
    assert!(timeline.contains("Time:"));
    let time = timeline
        .split(":")
        .last()
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<i64>()
        .expect("not a number");
    let distance_line = lines.next().unwrap();
    assert!(distance_line.contains("Distance:"));
    let distance = distance_line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<i64>()
        .expect("not a number");

    let race = Race { time, distance };
    race.get_win_ways_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_a() {
        let expected_output = 288;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let expected_output = 71503;
        assert_eq!(process_b(input), expected_output);
    }
}
