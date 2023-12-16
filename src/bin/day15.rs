//! Day 15 of Advent of Code
//!
//! Lens Library
//! ============
//!
//! This one looks simple enough,
//! it's just a simple hashing
//! algorithm to deal with...

use std::fs::read_to_string;

/// Input consists of a comma separated
/// list of strings to hash together.
///
/// For once, this function is infalliable!
fn parse_input<'i>(input: &'i str) -> Vec<&'i str> {
    input.trim().split(',').collect()
}

const FACTOR: u32 = 17;

const REDUCTION: u32 = 256;

fn hashvent(input: &str) -> u32 {
    input.chars().fold(0, |acc, ch| hashvent_step(ch, acc))
}

fn hashvent_step(ch: char, acc: u32) -> u32 {
    let hash: u32 = ch.into();
    let val = acc + hash;
    let val = val * FACTOR;
    val % REDUCTION
}

/// Part 1
/// ------
///
/// What is the sum of all of the hashes in the results?
fn part_one(data: &[&str]) -> u32 {
    data.iter().map(|&i| hashvent(i)).sum()
}

fn main() {
    let input = read_to_string("src/input/day15.txt").expect("Could not read input");
    let data = parse_input(&input);

    println!("The total sum of the hashes is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashvent() {
        assert_eq!(hashvent("HASH"), 52);
    }

    #[test]
    fn test_hashvent_step() {
        assert_eq!(hashvent_step('H', 0), 200);
        assert_eq!(hashvent_step('A', 200), 153);
        assert_eq!(hashvent_step('S', 153), 172);
        assert_eq!(hashvent_step('H', 172), 52);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day15-test.txt").expect("Could not read example");
        let data = parse_input(&input);

        assert_eq!(part_one(&data), 1320);
    }
}
