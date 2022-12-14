#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::dbgprint;

pub const TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

pub const TEST_RESULT_PART1: i32 = 2;
pub const TEST_RESULT_PART2: i32 = 4;

#[derive(Debug)]
struct Pair {
    first: (i32, i32),
    second: (i32, i32),
}
#[derive(Debug)]
struct Input { pairs: Vec<Pair> }

impl FromStr for Input {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let mut pairs: Vec<Pair> = Vec::new();
        for l in s.lines() {
            let pair_parts = l.split(',').collect::<Vec<_>>();
            let a = pair_parts[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            let b = pair_parts[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            pairs.push(Pair { first: (a[0], a[1]), second: (b[0], b[1]) })
        }

        Ok(Input { pairs: pairs })
    }
}

pub fn main() {
    //dbgprint::enable();

    println!("Advent of Code 2022");
    println!("Day 04 - Camp Cleanup");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, TEST_RESULT_PART1);
    assert_eq!(TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(std::fs::read_to_string("puzzles/day04_input.txt").unwrap().parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, TEST_RESULT_PART2);
    assert_eq!(TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(std::fs::read_to_string("puzzles/day04_input.txt").unwrap().parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let mut contained_pairs = 0;
    for p in input.pairs {
        if p.first.0 >= p.second.0 && p.first.1 <= p.second.1 {
            contained_pairs += 1;
        } else if p.second.0 >= p.first.0 && p.second.1 <= p.first.1 {
            contained_pairs += 1;
        }
    }
    contained_pairs
}

fn do_part2(input: Input) -> i32 {
    let mut overlapping_pairs = 0;
    for p in input.pairs {
        if p.first.0 >= p.second.0 && p.first.0 <= p.second.1 {
            overlapping_pairs += 1;
        } else if p.second.0 >= p.first.0 && p.second.0 <= p.first.1 {
            overlapping_pairs += 1;
        }
    }
    overlapping_pairs
}
