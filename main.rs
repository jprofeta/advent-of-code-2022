#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use]
mod dbgprint;

mod puzzles;

use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    match day {
        1 => puzzles::day01::main(),
        2 => puzzles::day02::main(),
        _ => println!("Undefined day")
    }
}
