use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::dbgprint;

pub const TEST_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

pub const TEST_RESULT_PART1: i32 = 24000;
pub const TEST_RESULT_PART2: i32 = 45000;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { elves: Vec<Vec<i32>> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let lines = s.lines();

        let mut elves: Vec<Vec<i32>> = Vec::new();
        let mut current_elf: Vec<i32> = Vec::new();
        for l in lines {
            if l.trim().len() == 0 {
                elves.push(current_elf.to_vec());
                current_elf = Vec::new();
            } else {
                let snack: i32 = l.trim().parse().expect("Need a number of calories");
                current_elf.push(snack);
            }
        }
        elves.push(current_elf.to_vec());

        Ok(Input { elves: elves })
    }
}

pub fn main() {
    //dbgprint::enable();

    println!("Advent of Code 2021");
    println!("Day 01 - Calorie Counting");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, TEST_RESULT_PART1);
    assert_eq!(TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(std::fs::read_to_string("puzzles/day01_input.txt").unwrap().parse::<Input>().unwrap());
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
    let puzzle_out2 = do_part2(std::fs::read_to_string("puzzles/day01_input.txt").unwrap().parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let max: i32 = input.elves.iter().map(|x| x.iter().sum()).max().unwrap();
    max
}

fn do_part2(input: Input) -> i32 {
    let mut total_per_elf: Vec<i32> = input.elves.iter().map(|x| x.iter().sum()).collect();
    total_per_elf.sort();
    total_per_elf.reverse();
    dbgprint!("{:?}", total_per_elf);
    total_per_elf.iter().take(3).sum()
}
