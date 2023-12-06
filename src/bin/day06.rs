//! Day 06 of Advent of Code 2023
//!
//! Wait For It
//! -----------
//! 
//! We're off to the races!

use std::fs::read_to_string;

use thiserror::Error;
use std::num::ParseIntError;

#[derive(Clone, Debug, Hash, PartialEq)]
struct Race(u32, u32);

#[derive(Clone, PartialEq, Debug, Error)]
enum ParseError {
    #[error("Failed to find {0} in input")]
    InvalidFormat(&'static str),
    #[error("Failed to parse number")]
    ParseFailed(#[from] ParseIntError),
}

/// Input today is comically simple.
///
/// It consists of two lines separated
fn parse_input(input: &str) -> Vec<Race> {
    unimplemented!();
}

fn part_one(data: &[Race]) -> u32 {
    unimplemented!();
}

#[allow(dead_code)]
fn part_two(data: &[Race]) {}

fn main() {
    let input = read_to_string("src/input/day06.txt").expect("Could not find input");
    let data = parse_input(&input);
}

#[cfg(test)]
mod test {}
