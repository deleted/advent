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
    unsafe { println!("Call count: {CALL_COUNT}") }
}

fn process_a(input: &str) -> i32 {
    let mut total_points = 0;
    for line in input.lines() {
        let mut sections = line
            .split(": ")
            .last()
            .expect("no colon in line")
            .split("|");

        let winning_numbers = sections
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("not a number"))
            .collect::<HashSet<i32>>();

        let card_numbers = sections
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("not a number"))
            .filter(|num| winning_numbers.contains(num))
            .collect::<Vec<i32>>();
        let mut points = 0;
        for _ in card_numbers {
            if points == 0 {
                points += 1;
            } else {
                points *= 2
            }
        }
        total_points += points;
    }
    total_points
}

#[derive(Debug)]
struct Card {
    idx: i32,
    winning_numbers: HashSet<i32>,
    matching_numbers: Vec<i32>,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let mut sections = line.split(": ");
        let card_idx = sections
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .expect("card idx not a number`");

        sections = sections.last().expect("no colon in line").split("|");

        let winning_numbers = sections
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("not a number"))
            .collect::<HashSet<i32>>();

        let matching_numbers = sections
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("not a number"))
            .filter(|num| winning_numbers.contains(num))
            .collect::<Vec<i32>>();

        Self {
            idx: card_idx,
            winning_numbers,
            matching_numbers,
        }
    }
}

fn process_b(input: &str) -> i32 {
    let mut cards = std::collections::BTreeMap::new();
    for line in input.lines() {
        let card = Card::from_line(line);
        cards.insert(card.idx, card);
    }

    let mut total = 0;
    for card in cards.values() {
        total += get_count(card, &cards, &Vec::new());
    }
    total
}

static mut COUNT_CACHE: BTreeMap<i32, i32> = BTreeMap::new();
static mut CALL_COUNT: i32 = 0;

fn get_count(card: &Card, cards: &BTreeMap<i32, Card>, path: &Vec<i32>) -> i32 {
    unsafe {
        match COUNT_CACHE.get(&card.idx) {
            Some(val) => return *val,
            None => (),
        }
    }
    let mut path = path.clone();
    path.push(card.idx);
    let mut total = 1; // one for this card
    let num_matches = card.matching_numbers.len() as i32;
    let my_idx = card.idx;
    for i in 0..num_matches {
        let next_idx = my_idx + 1 + i;
        let other_card = cards.get(&next_idx);
        match other_card {
            None => {
                println!("No card at {next_idx}")
            }
            Some(other_card) => total += get_count(other_card, cards, &path),
        }
    }
    unsafe {
        COUNT_CACHE.insert(my_idx, total);
        CALL_COUNT += 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let expected_output = 13;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let expected_output = 30;
        assert_eq!(process_b(input), expected_output);
    }
}
