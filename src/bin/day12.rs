//! Day 12 of Advent of Code
//!
//! Hot Springs
//! ===========
//! The springs are broken and
//! so are the records! We need
//! to fix this fast before
//! Gear Island panics! Or explodes!

use std::fs::read_to_string;
use std::iter::{repeat, zip};

use memoize::memoize;

use advent_2023::ParseError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Record {
    springs: Vec<SpringStatus>,
    errors: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    data.iter().map(solve).sum()
}

const FAILURE: u64 = 0;
const SUCCESS: u64 = 1;

#[memoize(Capacity: 256)]
/// Solve the given record, in a way that doesn't take forever.
///
/// TODO: This has an undiagnosed issue where it might give
/// odd results if the last element of `rec.springs` is not
/// an Okay. This is an easily fixed issue, but I should
/// figure out why it's happening in the first place.
fn smart_solve(rec: Record) -> u64 {
    // println!("{:?}", rec);
    let values = &rec.springs;
    let errors = &rec.errors;

    if errors.is_empty() {
        if values.contains(&SpringStatus::Broken) {
            return FAILURE;
        } else {
            return SUCCESS;
        }
    };

    // The minimum length of a possible record.
    let remaining_len = errors.iter().sum::<usize>() + errors.len();
    // If we don't have enough springs to satisfy the data,
    // bail out here.
    if values.len() < remaining_len {
        return FAILURE;
    }

    let mut possibilities = 0;

    if values[0] != SpringStatus::Broken {
        // Let's assume it's fine then.
        let yea = Record {
            springs: values[1..].to_owned(),
            errors: errors.clone(),
        };

        possibilities += smart_solve(yea);
    }

    let next_group = errors[0];
    // Does the group contain exactly enough errors?
    if !values[..next_group].contains(&SpringStatus::Okay)
        && values[next_group] != SpringStatus::Broken
    {
        // Then we can assume all of the relevant springs are damaged.
        let nay = Record {
            springs: values[next_group + 1..].to_vec(),
            errors: errors[1..].to_vec(),
        };

        possibilities += smart_solve(nay);
    }

    possibilities
}

/// Part 2
/// ------
///
/// So it turns out the input we have is incomplete. By
/// a factor of five. So, after duplicating the data we
/// have to fit the proper sizes, we answer the same
/// question: how many possibilities are there for the
/// data?
fn part_two(data: &[Record]) -> u64 {
    data.iter()
        .map(|rec| -> Record {
            let springs = repeat(rec.springs.clone())
                .take(5)
                .collect::<Vec<_>>()
                .join(&SpringStatus::Unknown);
            let springs: Vec<_> = [springs, vec![SpringStatus::Okay]].concat();

            let errors = repeat(rec.errors.clone())
                .take(5)
                .collect::<Vec<_>>()
                .concat();

            Record { springs, errors }
        })
        .map(smart_solve)
        .sum()
}

fn main() {
    let input = read_to_string("src/input/day12.txt").expect("Could not load input");
    let data = parse_input(&input).expect("Parsing failed");

    println!(
        "The total number of possible combinations is {}",
        part_one(&data)
    );
    println!(
        "The number of possibilities when unfolded is {}",
        part_two(&data)
    );
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

    #[test]
    fn test_smart_solve() {
        let input = read_to_string("src/input/day12-test.txt").expect("Could not load example");
        let data = parse_input(&input).expect("Parsing failed");

        let springs = [data[0].springs.clone(), vec![SpringStatus::Okay]].concat();
        let rec = Record {
            springs,
            errors: data[0].errors.clone(),
        };

        assert_eq!(smart_solve(rec), solve(&data[0]) as u64);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day12-test.txt").expect("Could not load example");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_two(&data), 525152);
    }
}
