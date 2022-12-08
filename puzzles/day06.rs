#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashSet;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::dbgprint;

pub const TEST_INPUT: &str = "\
mjqjpqmgbljsphdztnvjfqwrcgsmlb
";

pub const TEST_RESULT_PART1: i32 = 7;
pub const TEST_RESULT_PART2: i32 = 19;

#[derive(Debug)]
struct Input { data_stream: String }

impl FromStr for Input {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input { data_stream: String::from(s.trim()) })
    }
}

pub fn main() {
    //dbgprint::enable();

    println!("Advent of Code 2022");
    println!("Day 06 - Tuning Trouble");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, TEST_RESULT_PART1);
    assert_eq!(TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(std::fs::read_to_string("puzzles/day06_input.txt").unwrap().parse::<Input>().unwrap());
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
    let puzzle_out2 = do_part2(std::fs::read_to_string("puzzles/day06_input.txt").unwrap().parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let marker = input.data_stream
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .enumerate()
        .find(|(i, window)| {
            let mut window: Vec<char> = Vec::from_iter(window.iter().cloned());
            window.sort();
            window.dedup();
            window.len() == 4
        })
        .unwrap()
        .0;
    (marker as i32) + 4
}

fn do_part2(input: Input) -> i32 {
    let marker = input.data_stream
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .find(|(i, window)| {
            let mut window: Vec<char> = Vec::from_iter(window.iter().cloned());
            window.sort();
            window.dedup();
            window.len() == 14
        })
        .unwrap()
        .0;
    (marker as i32) + 14
}
