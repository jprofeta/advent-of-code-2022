#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::hash::Hash;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashSet;

use crate::dbgprint;
use crate::puzzles::day03_input;

#[derive(Debug)]
struct Input { rucksacks: Vec<String> }

impl FromStr for Input {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let rucksacks = s.lines().map(|l| String::from(l)).collect();

        Ok(Input { rucksacks: rucksacks })
    }
}

pub fn main() {
    dbgprint::enable();

    println!("Advent of Code 2021");
    println!("Day 03 - Rucksack Reorganization");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day03_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day03_input::TEST_RESULT_PART1);
    assert_eq!(day03_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day03_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day03_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day03_input::TEST_RESULT_PART2);
    assert_eq!(day03_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day03_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn priority(c: &char) -> i32 {
    let c = *c as u8;
    let lower_a = 'a' as u8;
    let upper_a = 'A' as u8;

    if c >= lower_a { (c - lower_a) as i32 + 1 }
    else { (c - upper_a) as i32 + 27 }
}

fn do_part1(input: Input) -> i32 {
    let dup_items: Vec<char> = input.rucksacks
        .iter()
        .map(|x| {
            let n = x.len()/2;
            (x[0..n].chars().collect::<HashSet<char>>(), x[n..].chars().collect::<HashSet<char>>())
        })
        .map(|(a,b)| 
            // Need to clone the option since the hashmaps are borrowed here.
            match a.intersection(&b).nth(0) {
                Some(x) => Some(x.clone()),
                None => None
            }
        )
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().clone())
        .collect();
    dbgprintln!("{:?}", dup_items);
    dup_items
        .iter()
        .map(|c| priority(&c))
        .sum()
}

fn do_part2(input: Input) -> i32 {
    let badge_items: Vec<char> =
        input.rucksacks
        .chunks(3)
        .map(|x| x.iter().map(|backpack| backpack.chars().collect::<HashSet<char>>()))
        .map(|mut g| {
            let first_intersection: HashSet<char> = g.nth(0).unwrap().intersection(&g.nth(0).unwrap()).map(|c| c.clone()).collect();
            first_intersection.intersection(&g.nth(0).unwrap()).map(|c| c.clone()).collect::<Vec<char>>()
        })
        .map(|x| x[0])
        .collect();
    dbgprintln!("{:?}", badge_items);
    badge_items
        .iter()
        .map(|c| priority(&c))
        .sum()
}
