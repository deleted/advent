use std::collections::BTreeMap;
use std::time;
use std::{fs::File, io::Read};

use clap::{arg, command, Parser};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct ParseResults {
    turns: Vec<Turn>,
    nodes: BTreeMap<String, (String, String)>,
}

impl ParseResults {
    fn from_str(input: &str) -> Self {
        let mut turns = Vec::new();
        let mut nodes: BTreeMap<String, (String, String)> = BTreeMap::new();

        for mut line in input.lines() {
            line = line.trim();
            if line.is_empty() {
                continue;
            }
            if turns.is_empty() {
                turns = line
                    .chars()
                    .map(|c| match c {
                        'L' => Turn::Left,
                        'R' => Turn::Right,
                        _ => panic!("Expected L or R"),
                    })
                    .collect();
                continue;
            }

            if line.contains("=") {
                let mut first_split = line.split("=").map(|s| s.trim());
                let key = first_split.next().unwrap();
                let mut leftright = first_split
                    .next()
                    .unwrap()
                    .trim_matches(|c| "()".contains(c))
                    .split(", ");
                let left = leftright.next().unwrap();
                let right = leftright.next().unwrap();
                nodes.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
            }
        }

        ParseResults { turns, nodes }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    node: String,
    turn: Turn,
}

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
    let parsed = ParseResults::from_str(input);
    let turns = parsed.turns;
    let mut count = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let turn = turns.get(count % turns.len()).unwrap();
        let (l, r) = &parsed.nodes.get(current_node).expect("missing node");
        current_node = match turn {
            Turn::Left => l,
            Turn::Right => r,
        };
        count += 1;
    }
    count as i32
}

fn process_b(input: &str) -> i64 {
    let parsed = ParseResults::from_str(input);
    let turns = parsed.turns;
    let nodes = parsed.nodes;
    let mut current_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();
    let mut time_to_terminate: Vec<Option<usize>> = vec![None; current_nodes.len()];
    let mut count = 0;
    loop {
        current_nodes.par_iter_mut().for_each(|node| {
            let (l, r) = nodes.get(*node).expect("missing node");
            let turn = turns.get(count % turns.len()).unwrap();
            *node = match turn {
                Turn::Left => l,
                Turn::Right => r,
            };
        });
        count += 1;

        current_nodes
            .iter()
            .map(|n| n.ends_with('Z'))
            .enumerate()
            .for_each(|(i, terminal)| {
                if terminal && time_to_terminate[i].is_none() {
                    println!("Starting node {} terminated at {}", i, count);
                    time_to_terminate[i] = Some(count);
                }
            });

        if time_to_terminate.iter().all(|t| t.is_some()) {
            println!("All cycles terminated.");
            break;
        }
    }

    assert!(time_to_terminate.iter().all(|t| t.is_some()));
    let times = time_to_terminate
        .iter()
        .map(|o| o.unwrap() as i64)
        .collect::<Vec<_>>();
    println!("Times: {:?}", &times);
    let l_c_m = times.iter().fold(1, |acc, b| lcm(&acc, b));
    l_c_m
}

fn lcm(a: &i64, b: &i64) -> i64 {
    let greater = *a.max(b);
    let lesser = *a.min(b);
    let mut accum = greater;
    loop {
        if accum % lesser == 0 {
            return accum;
        }
        accum += greater;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_a() {
        let expected_output = 6;

        assert_eq!(process_a(INPUT), expected_output);
    }

    #[test]
    fn test_b() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        let expected_output = 6;
        assert_eq!(process_b(input), expected_output);
    }
}
