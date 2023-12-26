//! Day 12 of Advent of Code
//!
//! Hot Springs
//! ===========
//! The springs are broken and
//! so are the records! We need
//! to fix this fast before
//! Gear Island panics! Or explodes!

use std::fs::read_to_string;
use std::iter::zip;

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

impl TryFrom<char> for SpringStatus {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Okay),
            '#' => Ok(Self::Broken),
            '?' => Ok(Self::Unknown),
            _ => Err(ParseError::InvalidFormat("valid spring character")),
        }
    }
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
    input
        .lines()
        .map(|line| {
            let Some((springs, errors)) = line.split_once(' ') else {
                return Err(ParseError::InvalidFormat("proper record format"));
            };

            let springs = springs
                .chars()
                .map(SpringStatus::try_from)
                .collect::<Result<Vec<_>, ParseError>>()?;

            let errors = errors
                .split(',')
                .map(|err| err.parse().map_err(ParseError::ExpectedNumber))
                .collect::<Result<Vec<usize>, ParseError>>()?;

            Ok(Record { springs, errors })
        })
        .collect::<Result<Vec<Record>, ParseError>>()
}

/// A recursive function to solve the given record.
fn solve(rec: &Record) -> u32 {
    let springs = &rec.springs;
    match springs
        .iter()
        .position(|spring| *spring == SpringStatus::Unknown)
    {
        Some(position) => {
            // Solve for the unknown spring being okay and broken.
            let mut yea = rec.clone();
            yea.springs[position] = SpringStatus::Okay;

            let mut nay = rec.clone();
            nay.springs[position] = SpringStatus::Broken;

            solve(&yea) + solve(&nay)
        }
        None => {
            // println!("{:?}", springs);
            let map = springs
                // Get all of the runs of broken springs
                .split(|spring| *spring == SpringStatus::Okay)
                // Remove any empty lists
                .filter(|list| !list.is_empty())
                // Get the length of all of the lists
                .map(|list| list.len())
                // Collect into Vec for length check
                .collect::<Vec<_>>();

            // Check for equal length...
            if map.len() == rec.errors.len()
                // ...and equal elements.
                && zip(map, &rec.errors).all(|(actual, &expected)| actual == expected)
            {
                1
            } else {
                0
            }
        }
    }
}

/// Part 1
/// ------
///
/// Given the information we have, how many possibilities
/// are there that satisfy the constraints?
fn part_one(data: &[Record]) -> u32 {
    data.iter().map(|record| solve(record)).sum()
}

#[allow(unused)]
fn part_two(data: &[Record]) {
    todo!();
}

fn main() {
    let input = read_to_string("src/input/day12.txt").expect("Could not load input");
    let data = parse_input(&input).expect("Parsing failed");

    println!("The total number of possible combinations is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day12-test.txt").expect("Could not load example");
        let data = parse_input(&input).expect("Parsing failed");

        let expected = Record {
            springs: vec![
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Okay,
                SpringStatus::Broken,
                SpringStatus::Broken,
                SpringStatus::Broken,
            ],
            errors: vec![1, 1, 3],
        };

        assert_eq!(expected, data[0]);
    }

    #[test]
    fn test_solve() {
        let input = read_to_string("src/input/day12-test.txt").expect("Could not load example");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(solve(&data[0]), 1);
        assert_eq!(solve(&data[1]), 4);
        assert_eq!(solve(&data[5]), 10);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day12-test.txt").expect("Could not load example");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_one(&data), 21);
    }
}
