//! Day 04 of Advent of Code 2023

use std::fs::read_to_string;

fn parse_input(input: &str) {}

fn part_one() {}

#[allow(dead_code)]
fn part_two() {
    todo!("Part one incomplete!");
}

fn main() {
    let input = read_to_string("src/input/day04.txt").expect("Could not find input");
    let data = parse_input(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day04-test.txt").expect("Could not find example");
    }
}
