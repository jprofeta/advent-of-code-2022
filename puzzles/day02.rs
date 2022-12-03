#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::dbgprint;
use crate::puzzles::day02_input;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { rounds: Vec<(Hand,Hand)> }

enum Hand {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let lines = s.lines();

        Ok(Input { })
    }
}

pub fn main() {
    //dbgprint::enable();

    println!("Advent of Code 2021");
    println!("Day 02 - Rock Paper Scissors");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day02_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day02_input::TEST_RESULT_PART1);
    assert_eq!(day02_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day02_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day02_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day02_input::TEST_RESULT_PART2);
    assert_eq!(day02_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day02_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    0
}

fn do_part2(input: Input) -> i32 {
    0
}
