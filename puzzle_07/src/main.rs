use core::num;
use std::collections::BTreeMap;
use std::iter::Iterator;
use std::time;
use std::{collections::HashSet, fs::File, future::IntoFuture, io::Read};

use clap::{arg, command, Parser};
use counter::Counter;
use itertools::Itertools;

const CARDS: &str = "AKQJT98765432";

fn strength(c: char) -> Option<i32> {
    let idx = CARDS.find(c);
    match idx {
        None => None,
        Some(i) => Some((CARDS.len() - i) as i32),
    }
}

fn strength_part2(c: char) -> Option<i32> {
    let cards: &str = "AKQT98765432J";
    let idx = cards.find(c).expect("not a real card");
    Some((cards.len() - idx) as i32)
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
enum Ruleset {
    A,
    B,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
    ruleset: Ruleset,
}

impl Hand {
    fn from_str(input: &str, ruleset: Ruleset) -> Self {
        let mut toks = input.trim().split_whitespace();
        let cards = toks.next().unwrap().chars().collect::<Vec<char>>();
        let bid = toks.next().unwrap().parse::<i32>().expect("not a number");
        if cards.len() != 5 {
            panic!("Wrong number of cards: {}", cards.len())
        };
        for card in &cards {
            if !CARDS.contains(*card) {
                panic!("{} is not a card", card)
            }
        }
        Self {
            cards,
            bid,
            ruleset,
        }
    }

    fn compute_type(&self) -> HandType {
        let mut counter = self.cards.iter().collect::<Counter<_>>();
        let mut by_common = counter.most_common();
        let mut j_count = *match counter.get(&'J') {
            Some(count) => count,
            None => &0,
        } as i32;

        if self.ruleset == Ruleset::B {
            while j_count > 0 {
                let mostkey = by_common[0].0;
                *counter.get_mut(mostkey).unwrap() += 1;
                *counter.get_mut(&'J').unwrap() -= 1;
                j_count -= 1;
                by_common = counter.most_common();
            }
        }

        let hand_type = match by_common[0].1 {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match by_common[1].1 {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match by_common[1].1 {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        };
        hand_type
    }

    fn card_strengths(&self) -> Vec<i32> {
        let strength_func = match self.ruleset {
            Ruleset::A => strength,
            Ruleset::B => strength_part2,
        };
        return self
            .cards
            .iter()
            .map(|c| strength_func(*c).unwrap())
            .collect::<Vec<_>>();
    }

    fn sort_key(&self) -> Vec<i32> {
        let mut key = Vec::new();
        key.push(self.compute_type() as i32);
        let mut scores = self.card_strengths();
        scores.iter().for_each(|v| key.push(*v));
        key
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.compute_type() == other.compute_type() && self.cards[0] == other.cards[0]
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = self.compute_type();
        let other_type = other.compute_type();
        if self_type == other_type {
            dbg!("partial_cmp()");
            return self.card_strengths().partial_cmp(&other.card_strengths());
        } else {
            return self_type.partial_cmp(&other_type);
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.compute_type();
        let other_type = other.compute_type();
        if self_type == other_type {
            dbg!("cmp()");
            return self.card_strengths().cmp(&other.card_strengths());
        } else {
            return self_type.cmp(&other_type);
        }
    }
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
    let mut hands = input
        .lines()
        .map(|line| Hand::from_str(line, Ruleset::A))
        .collect::<Vec<_>>();
    hands.sort();
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i as i32 + 1)
    }
    total
}

fn process_b(input: &str) -> i32 {
    let mut hands = input
        .lines()
        .map(|line| Hand::from_str(line, Ruleset::B))
        .collect::<Vec<_>>();
    // hands.sort_unstable_by(|a, b| a.sort_key().cmp(&b.sort_key()));
    hands.sort_by(|a, b| {
        let key_a = a.sort_key();
        let key_b = b.sort_key();
        for i in 0..key_a.len() {
            if key_a[i] != key_b[i] {
                return key_a[i].cmp(&key_b[i]);
            }
        }
        return std::cmp::Ordering::Equal;
    });
    hands.iter().for_each(|h| {
        // dbg!(&h.cards, h.sort_key(), h.compute_type());
        println!("{:?}", h.sort_key());
    });
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i as i32 + 1)
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static input: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test_a() {
        let expected_output = 6440;

        assert_eq!(process_a(input), expected_output);
    }

    #[test]
    fn test_b() {
        let expected_output = 5905;
        assert_eq!(process_b(input), expected_output);
    }

    #[test]
    fn test_card_strength() {
        assert!(strength('A') > strength('K'));
        assert!(strength('3') > strength('2'));
    }

    #[test]
    fn test_card_strength2() {
        assert!(strength_part2('K') > strength_part2('Q'));
        assert!(strength_part2('Q') > strength_part2('T'));
    }

    #[test]
    fn test_hand_strength() {
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
    }

    #[test]
    fn test_lexical_ordering() {
        let a = vec![6, 12, 10, 1, 1, 10];
        let b = vec![6, 11, 11, 11, 1, 13];
        // let a = vec![6, 12];
        // let b = vec![6, 11];
        assert!(a > b);
        let mut sortable = vec![&a, &b];
        sortable.sort();
        assert_eq!(sortable, vec![&b, &a]);
    }
}
