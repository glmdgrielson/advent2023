//! Day 12 of Advent of Code
//!
//! Hot Springs
//! ===========
//! The springs are broken and
//! so are the records! We need
//! to fix this fast before
//! Gear Island panics! Or explodes!

use std::fs::read_to_string;

use advent_2023::ParseError;

#[derive(Clone, Debug, PartialEq)]
struct Record {
    springs: Vec<SpringStatus>,
    errors: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SpringStatus {
    Unknown,
    Broken,
    Okay,
}

/// Input consists of a series of records.
///
/// Each record consists of a line of characters,
/// either `.`, `#`, or `?`, signifying an okay
/// spring, a broken spring, or a spring of
/// unknown status. This is then followed up by
/// a list of the lengths of each row of broken
/// springs for that record, determined by
/// the rules of Paint by Numbers puzzles.
fn parse_input(input: &str) -> Result<Vec<Record>, ParseError> {
    todo!();
}

fn part_one(data: &[Record]) -> u32 {
    todo!();
}

#[allow(unused)]
fn part_two(data: &[Record]) {
    todo!();
}

fn main() {
    let input = read_to_string("src/input/day12.txt").expect("Could not load input");
    let data = parse_input(&input).expect("Parsing failed");

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day12-test.txt").expect("Could not load example");
        let data = parse_input(&input).expect("Parsing failed");
    }
}
