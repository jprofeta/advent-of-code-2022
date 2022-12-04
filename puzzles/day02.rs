#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::num::Wrapping;

use crate::dbgprint;

pub const TEST_INPUT: &str = "\
A Y
B X
C Z
";

pub const TEST_RESULT_PART1: i32 = 15;
pub const TEST_RESULT_PART2: i32 = 12;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { rounds: Vec<(InValue, OutValue)> }

#[derive(Debug, Clone, Eq, PartialEq, Copy, PartialOrd)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, PartialOrd)]
enum InValue {
    A,
    B,
    C
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, PartialOrd)]
enum OutValue {
    X,
    Y,
    Z
}

impl FromStr for Input {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let lines = s.lines();
        let mut rounds: Vec<(InValue,OutValue)> = Vec::new();
        for l in lines {
            let parts = l.split(' ').collect::<Vec<&str>>();
            let in_value = match parts[0] {
                "A" => InValue::A,
                "B" => InValue::B,
                "C" => InValue::C,
                _ => return Err("bad input value")
            };
            let out_value = match parts[1] {
                "X" => OutValue::X,
                "Y" => OutValue::Y,
                "Z" => OutValue::Z,
                _ => return Err("bad input value")
            };
            rounds.push((in_value, out_value))
        }
        Ok(Input { rounds: rounds.to_vec() })
    }
}

impl TryFrom<InValue> for Hand {
    type Error = &'static str;
    fn try_from(x: InValue) -> Result<Self, Self::Error> {
        match x {
            InValue::A => Ok(Hand::Rock),
            InValue::B => Ok(Hand::Paper),
            InValue::C => Ok(Hand::Scissors)
        }
    }
}

impl TryFrom<OutValue> for Hand {
    type Error = &'static str;
    fn try_from(x: OutValue) -> Result<Self, Self::Error> {
        match x {
            OutValue::X => Ok(Hand::Rock),
            OutValue::Y => Ok(Hand::Paper),
            OutValue::Z => Ok(Hand::Scissors)
        }
    }
}

pub fn main() {
    dbgprint::enable();

    println!("Advent of Code 2021");
    println!("Day 02 - Rock Paper Scissors");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, TEST_RESULT_PART1);
    assert_eq!(TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(std::fs::read_to_string("puzzles/day02_input.txt").unwrap().parse::<Input>().unwrap());
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
    let puzzle_out2 = do_part2(std::fs::read_to_string("puzzles/day02_input.txt").unwrap().parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn score_part1(a: &InValue, b: &OutValue) -> i32 {
    let a = Hand::try_from(*a).unwrap();
    let b = Hand::try_from(*b).unwrap();
    let _b = b as i32;
    
    // Winning condition
    if (b == Hand::Paper && a == Hand::Rock)
        || (b == Hand::Scissors && a == Hand::Paper)
        || (b == Hand::Rock && a == Hand::Scissors) {
        dbgprintln!("W: {:?} vs {:?}", b, a);
        _b + 6
    }
    // Tie
    else if b == a {
        dbgprintln!("T: {:?} vs {:?}", b, a);
        _b + 3
    }
    // Lose
    else {
        dbgprintln!("L: {:?} vs {:?}", b, a);
        _b + 0
    }
}

fn score_part2(a: &InValue, b: &OutValue) -> i32 {
    let a = Hand::try_from(*a).unwrap();
    let _a = a as i32;

    match *b {
        // Lose
        OutValue::X => if a == Hand::Rock { Hand::Scissors as i32 } else { _a - 1 }
        // Tie
        OutValue::Y => _a + 3,
        // Win
        // There is an implicit _a + 1 because _a is 1 based. This removes the need to -1 then +1 to rebase it
        // for the modulus.
        OutValue::Z => _a % 3 + 1 + 6
    }
}

fn do_part1(input: Input) -> i32 {
    let scores: Vec<i32> =
        input.rounds.iter()
        .map(|(a,b)| score_part1(a,b))
        .collect();
        dbgprintln!("{:?}", scores);
    scores.iter().sum()
}

fn do_part2(input: Input) -> i32 {
    let scores: Vec<i32> =
        input.rounds.iter()
        .map(|(a,b)| score_part2(a,b))
        .collect();
        dbgprintln!("{:?}", scores);
    scores.iter().sum()
}
