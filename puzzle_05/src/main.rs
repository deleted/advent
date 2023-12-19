use std::iter::Iterator;
use std::time;
use std::{fs::File, io::Read};

use clap::{arg, command, Parser};
use itertools::Itertools;
use rayon::prelude::*;

// Time to beat:
// Result B: 10834440 in 2192.901100744s (brute force serial)
// Result B: 10834440 in 880.852927328s (brute force parallel with rayon)

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
struct MapRange {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

impl MapRange {
    fn resolve(&self, val: i64) -> Option<i64> {
        if val >= self.source_start && val < self.source_start + self.len {
            return Some(self.dest_start + (val - self.source_start));
        } else {
            return None;
        }
    }

    fn source_range(&self) -> std::ops::Range<i64> {
        self.source_start..(self.source_start + self.len)
    }

    fn min_overlap(&self, start: i64, len: i64) -> Option<i64> {
        let my_range = self.source_range();
        let end = start + len;
        if my_range.contains(&start) {
            return Some(start);
        } else if my_range.contains(&end) {
            return Some(my_range.start);
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
struct AlmanacMap {
    name: String,
    ranges: Vec<MapRange>,
}

impl AlmanacMap {
    fn from_tuples(name: &str, tuples: &Vec<(i64, i64, i64)>) -> Self {
        let mut ranges = Vec::new();
        for tup in tuples {
            let (dest_start, source_start, range_len) = *tup;
            ranges.push(MapRange {
                source_start,
                dest_start,
                len: range_len,
            })
        }
        Self {
            name: name.to_string(),
            ranges,
        }
    }

    /// return the stored mapped value if it's in the map,
    /// other wise just return the input
    fn get(&self, key: i64) -> i64 {
        for range in &self.ranges {
            match range.resolve(key) {
                Some(val) => return val,
                None => continue,
            }
        }
        return key;
    }

    fn min_for_range(&self, start: i64, len: i64) {}
}

fn process_a(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    assert!(seeds_line.starts_with("seeds:"));
    let seeds = seeds_line
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter(|tok| !tok.is_empty())
        .map(|tok| tok.parse::<i64>().expect("seed not a number"))
        .collect::<Vec<i64>>();

    let mut current_map_name = "";
    let mut current_tuples = Vec::new();
    let mut maps = Vec::new();

    for line in lines {
        // dbg!(current_map_name);
        if line.contains("map:") {
            current_map_name = line.trim().split(" ").next().unwrap();
            dbg!(current_map_name);
            continue;
        } else if line.trim().is_empty() {
            dbg!(current_map_name);
            if !current_map_name.is_empty() {
                maps.push(AlmanacMap::from_tuples(current_map_name, &current_tuples));
                current_tuples.clear();
                current_map_name = "";
            }
            continue;
        } else {
            let tup = line
                .split(" ")
                .filter(|tok| !tok.is_empty())
                .take(3)
                .map(|tok| tok.parse::<i64>().expect("map element not a number"))
                .next_tuple::<(i64, i64, i64)>()
                .unwrap();
            current_tuples.push(tup);
        }
    }
    if !current_map_name.is_empty() {
        maps.push(AlmanacMap::from_tuples(current_map_name, &current_tuples));
        current_tuples.clear();
        current_map_name = "";
    }

    let mut results = Vec::new();
    for seed in seeds {
        let map_iter = maps.iter();
        let loc = map_iter.fold(seed, |acc, m| m.get(acc));
        results.push(loc);
    }
    let min_loc = results.iter().reduce(|acc, n| acc.min(n)).unwrap();
    *min_loc
}

fn process_b(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    assert!(seeds_line.starts_with("seeds:"));
    let seeds = seeds_line
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter(|tok| !tok.is_empty())
        .map(|tok| tok.parse::<i64>().expect("seed not a number"))
        .collect::<Vec<i64>>();

    let mut current_map_name = "";
    let mut current_tuples = Vec::new();
    let mut maps = Vec::new();

    for line in lines {
        // dbg!(current_map_name);
        if line.contains("map:") {
            current_map_name = line.trim().split(" ").next().unwrap();
            dbg!(current_map_name);
            continue;
        } else if line.trim().is_empty() {
            dbg!(current_map_name);
            if !current_map_name.is_empty() {
                maps.push(AlmanacMap::from_tuples(current_map_name, &current_tuples));
                current_tuples.clear();
                current_map_name = "";
            }
            continue;
        } else {
            let tup = line
                .split(" ")
                .filter(|tok| !tok.is_empty())
                .take(3)
                .map(|tok| tok.parse::<i64>().expect("map element not a number"))
                .next_tuple::<(i64, i64, i64)>()
                .unwrap();
            current_tuples.push(tup);
        }
    }
    if !current_map_name.is_empty() {
        maps.push(AlmanacMap::from_tuples(current_map_name, &current_tuples));
        current_tuples.clear();
        current_map_name = "";
    }

    // let mut results = Vec::new();
    // let mut min_result = std::i64::MAX;
    // for pair in seeds.par_chunks_exact(2) {
    let min_result = seeds
        .par_chunks_exact(2)
        .map(|pair| {
            let mut min_result = i64::MAX;
            let range = pair[0]..(pair[0] + pair[1]);
            dbg!(&range);
            for seed in range {
                let map_iter = maps.iter();
                let loc = map_iter.fold(seed, |acc, m| m.get(acc));
                if loc < min_result {
                    min_result = loc
                }
            }
            dbg!(min_result);
            min_result
        })
        .reduce(|| i64::MAX, |a, b| a.min(b));
    // let min_loc = results.iter().reduce(|acc, n| acc.min(n)).unwrap();
    min_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let expected_output = 35;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let expected_output = 46;
        assert_eq!(process_b(input), expected_output);
    }
}
