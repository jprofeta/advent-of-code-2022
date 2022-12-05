#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::dbgprint;

pub const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

pub const TEST_RESULT_PART1: &str = "CMZ";
pub const TEST_RESULT_PART2: &str = "MCD";

#[derive(Debug)]
struct Move { num: i32, src: usize, dest: usize }
#[derive(Debug)]
struct Input { stacks: Vec<Vec<char>>, moves: Vec<Move> }

fn read_move(s: &&str) -> Move {
    let from_idx = s.find(" from").expect("move must have origin");
    let to_idx = s.find(" to").expect("move must have destination");

    let num_crates: i32 = s[5..from_idx].parse().expect("invalid number of crates");
    let src: usize = s[from_idx+6..to_idx].parse().expect("invalid source stack");
    let dest: usize = s[to_idx+4..].parse().expect("invalid destination stack");

    Move { num: num_crates, src: src, dest: dest }
}

impl FromStr for Input {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = s.lines()
            .collect::<Vec<_>>()
            .split(|line| line.is_empty())
            .map(Vec::from)
            .collect();
        let crates = &sections[0];
        let moves: Vec<Move> = sections[1].iter().map(read_move).collect();

        let num_stacks = crates[crates.len()-1]
            .split_whitespace()
            .collect::<Vec<_>>().iter()
            .map(|n| n.parse::<i32>().expect("invalid stack number"))
            .max()
            .unwrap();

        let mut stacks = vec![Vec::<char>::new(); num_stacks as usize];

        for line in crates[0..crates.len()-1].iter() {
            for cr in line.match_indices('[') {
                let stack = cr.0 / 4;
                let crate_name = line.chars().nth(cr.0+1).unwrap();
                stacks[stack].push(crate_name);
            }
        }

        Ok(Input { stacks: stacks, moves: moves })
    }
}

pub fn main() {
    dbgprint::enable();

    println!("Advent of Code 2021");
    println!("Day 05 - Supply Stacks");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, TEST_RESULT_PART1);
    assert_eq!(TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(std::fs::read_to_string("puzzles/day05_input.txt").unwrap().parse::<Input>().unwrap());
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
    let puzzle_out2 = do_part2(std::fs::read_to_string("puzzles/day05_input.txt").unwrap().parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> String {
    let mut stacks = input.stacks.clone();
    for mv in input.moves {
        for _ in 0..mv.num {
                let x = stacks[mv.src - 1].remove(0);
                stacks[mv.dest - 1].insert(0, x)
        }
    }

    String::from_iter(stacks.iter().map(|stack| stack.first().unwrap_or(&' ')))
}

fn do_part2(input: Input) -> String {
    let mut stacks = input.stacks.clone();
    for mv in input.moves {
        let moved_crates: Vec<_> = stacks[mv.src - 1]
            .drain(0..mv.num as usize)
            .collect();
        moved_crates
            .iter()
            .enumerate()
            .for_each(|(i, val)| stacks[mv.dest - 1].insert(i, *val));
    }

    String::from_iter(stacks.iter().map(|stack| stack.first().unwrap_or(&' ')))
}
