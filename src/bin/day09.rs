//! Day 09 of Advent of Code 2023
//!
//! Mirage Maintenance
//! ==================
//!
//! Oh we're doing Sierpinksi triangle
//! nonsense...

use std::fs::read_to_string;

use advent_2023::ParseError;

#[derive(Clone, Debug, PartialEq)]
struct Report {}

fn parse_input(input: &str) -> Report {
    unimplemented!();
}

fn part_one(data: Report) {
    unimplemented!();
}

#[allow(unused)]
fn part_two(data: Report) {
    unimplemented!();
}

fn main() {
    let input = read_to_string("src/input/day09.txt").expect("Could not read data");
    let data = parse_input(&input);
}

#[cfg(test)]
mod test {
    use super::*;

}
