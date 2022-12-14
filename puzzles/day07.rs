#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::string;

use crate::dbgprint;

pub const TEST_INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

pub const TEST_RESULT_PART1: i32 = 95437;
pub const TEST_RESULT_PART2: i32 = 24933642;

const START_OF_COMMAND: &str = "$ ";

#[derive(Debug)]
enum Command {
    Cd,
    Ls
}

#[derive(Debug)]
struct Execution {
    command: Command,
    args: Vec<String>,
    stdout: String
}

impl FromStr for Execution {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, stdout) = s.split_once(|c| (c == '\n') || (c == '\r')).expect("Needs a command followed by output.");
        let (exe, args) = s.split_once(char::is_whitespace).expect("Needs an executable and arguments separated by a space.");
        let command = match exe.trim().to_lowercase().as_str() {
            "ls" => Command::Ls,
            "cd" => Command::Cd,
            x => { eprintln!("Unknown command: {}", x); return Err("Unknown command type") }
        };
        Ok(Execution {
            command: command,
            args: args.split(char::is_whitespace).map(String::from).collect(),
            stdout: String::from(stdout)
        })
    }
}

#[derive(Debug)]
struct Input { commands: Vec<Execution> }

impl FromStr for Input {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let commands: Vec<_> = 
            s
            .split(START_OF_COMMAND)
            .filter(|s| !str::is_empty(s))
            .map(Execution::from_str)
            .map(|x| x.expect("Valid command execution"))
            .collect();

        Ok(Input { commands: commands })
    }
}

pub fn main() {
    dbgprint::enable();

    println!("Advent of Code 2022");
    println!("Day 07 - No Space Left On Device");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, TEST_RESULT_PART1);
    assert_eq!(TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(std::fs::read_to_string("puzzles/day07_input.txt").unwrap().parse::<Input>().unwrap());
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
    let puzzle_out2 = do_part2(std::fs::read_to_string("puzzles/day07_input.txt").unwrap().parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn get_dir_size(file_entries: &HashMap<String, usize>, path: &String) -> usize {
    file_entries
    .iter()
    .filter(|(k,v)| k.starts_with(path) & !k.ends_with("/"))
    .map(|(k,v)| v).sum()
}

fn populate_listing(file_entries: &mut HashMap<String, usize>, commands: &Vec<Execution>) {
    let mut path = String::from("");
    for cmd in commands {
        match cmd.command {
            Command::Cd => {
                match cmd.args[0].as_str() {
                    // Both .. and / end a directory and cause it to have its size calculated, but the end location is different.
                    ".." => {
                        file_entries.insert(path.clone(), 0);
                        
                        let dir = &path[0..path.len() - 1];
                        let last_slash_idx = dir.rfind("/").expect("At top of tree, cannot move up anymore.");
                        path = dir[0..last_slash_idx + 1].to_string();
                    },
                    "/" => {
                        if path != "" {
                            file_entries.insert(path.clone(), 0);
                        }

                        path = String::from("/");
                    },
                    dir => {
                        path.push_str(dir);
                        path.push_str("/");
                    }
                }
                dbgprintln!("> {}", path);
            },
            Command::Ls => {
                let listings: Vec<_> = cmd.stdout
                    .lines()
                    .map(|line| line.split_once(" ").expect("two values separated by space for file listing."))
                    .map(|(a,b)| match a { "dir" => (b.to_string() + "/", 0), _ => (b.to_string(), a.parse::<usize>().expect("Valid file size")) })
                    .collect();
                for (item, size) in listings {
                    let item_path = path.clone() + &item;
                    file_entries.insert(item_path, size);
                }
            }
        }
    }

    // Push the last path
    file_entries.insert(path.clone(), 0);

    // Push the root
    file_entries.insert("/".to_string(), 0);

    // Fill in the size for directories
    let directories: Vec<_> = file_entries
        .iter()
        .filter(|(k,_)| k.ends_with("/"))
        .map(|(k,_)| k.clone())
        .collect();
    
    for dir in directories {
        let size = get_dir_size(file_entries, &dir);
        file_entries.insert(dir.clone(), size);
    }
}

fn do_part1<'a>(input: Input) -> i32 {
    let mut file_entries: HashMap<String, usize> = HashMap::new();
    populate_listing(&mut file_entries, &input.commands);

    file_entries
    .iter()
    .inspect(|(path,size)| dbgprintln!("{}\t{:?}", path, size))
    .filter(|(k,_)| k.ends_with("/"))
    .inspect(|(path,size)| dbgprintln!("{}\t{:?}", path, size))
    .map(|(_,v)| *v as i32)
    .filter(|size| size <= &100_000)
    .sum()
}

const FILESYSTEM_SIZE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

fn do_part2(input: Input) -> i32 {
    let mut file_entries: HashMap<String, usize> = HashMap::new();
    populate_listing(&mut file_entries, &input.commands);

    let total_used = file_entries.get("/").expect("Root element is required.");

    let space_needed = NEEDED_SPACE - (FILESYSTEM_SIZE - total_used);
    file_entries
        .iter()
        .filter(|(k,_)| k.ends_with("/"))
        .map(|(_,v)| *v)
        .filter(|size| size >= &space_needed)
        .min()
        .expect("Minimum largest directory.")
        as i32
}
