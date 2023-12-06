//! Day 06 of Advent of Code 2023
//!
//! Wait For It
//! -----------
//!
//! We're off to the races!

use std::fs::read_to_string;

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Clone, Debug, Hash, PartialEq)]
/// A race of the toy boats.
///
/// The first number is the time in milliseconds
/// of the race, and the second is the distance
/// record we want to beat in the race.
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
/// It consists of two lines each consisting
/// of a label and a series of numbers separated
/// by whitespace.
fn parse_input(input: &str) -> Result<Vec<Race>, ParseError> {
    let Some((time, distance)) = input.split_once("\n") else {
        return Err(ParseError::InvalidFormat("two lines of data"));
    };

    let Some(time) = time.strip_prefix("Time: ") else {
        return Err(ParseError::InvalidFormat("'Time:' tag"));
    };
    let Some(distance) = distance.strip_prefix("Distance: ") else {
        return Err(ParseError::InvalidFormat("'Distance:' tag"));
    };

    let time = time
        .trim()
        .split_whitespace()
        .map(|t| t.parse::<u32>().map_err(ParseError::ParseFailed))
        .collect::<Result<Vec<u32>, ParseError>>()?;
    let distance = distance
        .trim()
        .split_whitespace()
        .map(|t| t.parse::<u32>().map_err(ParseError::ParseFailed))
        .collect::<Result<Vec<u32>, ParseError>>()?;

    if time.len() != distance.len() {
        Err(ParseError::InvalidFormat("balanced races"))
    } else {
        Ok(time
            .iter()
            .zip(distance.iter())
            .map(|(time, distance)| Race(*time, *distance))
            .collect())
    }
}

/// Part 1
/// ------
///
/// If we want to compete in a race, we have to
/// activate our boat for a certain amount of
/// time and then beat the distance record with
/// whatever time we have remaining.
///
/// For each race, how many ways can we win?
fn part_one(data: &[Race]) -> usize {
    data.iter()
        .map(|race| {
            let total_time = race.0;
            let record = race.1;
            (0..total_time - 1)
                .map(|charge| charge * (total_time - charge))
                .filter(|&distance| distance > record)
                .count()
        })
        .product()
}

/// Part 2
/// ------
///
/// Stupid elves misprinting their signs...
///
/// So it turns out that there's only _one_
/// race, and naturally that race has frelling huge
/// numbers to deal with. Still the question is the
/// same: how many ways can we win?
fn part_two(data: &[Race]) {}

fn main() {
    let input = read_to_string("src/input/day06.txt").expect("Could not find input");
    let data = parse_input(&input).expect("Parsing must succeed");

    println!("The product of our victories is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day06-test.txt").expect("Could not find example");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert_eq!(data.first(), Some(&Race(7, 9)));
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day06-test.txt").expect("Could not find example");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert_eq!(part_one(&data), 288);
    }
}
