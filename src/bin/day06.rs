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
    // Our input is supposed to be exactly two lines long,
    // so this check is an easy one to make.
    let Some((time, distance)) = input.split_once("\n") else {
        return Err(ParseError::InvalidFormat("two lines of data"));
    };

    // Remove the textual prefixes so we have just the numbers.
    let Some(time) = time.strip_prefix("Time: ") else {
        return Err(ParseError::InvalidFormat("'Time:' tag"));
    };
    let Some(distance) = distance.strip_prefix("Distance: ") else {
        return Err(ParseError::InvalidFormat("'Distance:' tag"));
    };

    let time = time
        // Remove whatever whitespace was before the first number
        .trim()
        // Reduce the string to just the list of numbers
        .split_whitespace()
        // Convert the numbers into integers
        .map(|t| t.parse::<u32>().map_err(ParseError::ParseFailed))
        // Bail if the conversion failed at any point.
        .collect::<Result<Vec<u32>, ParseError>>()?;
    // Run the same algorithm again for the other set of numbers.
    let distance = distance
        .trim()
        .split_whitespace()
        .map(|t| t.parse::<u32>().map_err(ParseError::ParseFailed))
        .collect::<Result<Vec<u32>, ParseError>>()?;

    // Part one of the puzzle relies on each time
    // having a matching distance, so return an
    // error if that winds up not being the case.
    if time.len() != distance.len() {
        Err(ParseError::InvalidFormat("balanced races"))
    } else {
        // Otherwise, we have our data and we're off
        // to the races, quite literally in this case.
        Ok(time
            .iter()
            // Pair the times up with the distances.
            .zip(distance.iter())
            // Return the pair data.
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
        // For each race...
        .map(|race| {
            let total_time = race.0;
            let record = race.1;
            // And for every possible amount of charge time
            // besides the two that are guaranteed to be
            // zero (no charge time and all charge time)...
            (1..total_time - 1)
                // Get the total amount of distance traveled...
                .map(|charge| charge * (total_time - charge))
                // ...and see if it beats the distance record
                .filter(|&distance| distance > record)
                // Return the number of times we do in fact
                // win the race.
                .count()
        })
        // Multiply the scores of all of the races
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
fn part_two(data: &[Race]) -> Result<usize, ParseIntError> {
    let total_time = data
        .iter()
        // Convert each number BACK
        // into a string
        .map(|race| race.0.to_string())
        // Convert the iterator into a Vec
        // so we have access to a useful method
        .collect::<Vec<_>>()
        // Concatenate all of the strings
        .concat()
        // Convert the string into a number.
        //
        // This is a u64 because it turns out
        // that this number is frelling huge. Eep.
        .parse::<u64>()?;
    // Run the same algorithm for the distance record.
    let record = data
        .iter()
        .map(|race| race.1.to_string())
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>()?;

    // This is the same algorithm we saw in part one.
    //
    // I could _probably_ extract this into a function,
    // now that I think of it, but that's a "for later"
    // optimization that I probably don't need to make.
    Ok((1..total_time - 1)
        .map(|charge| charge * (total_time - charge))
        .filter(|&distance| distance > record)
        .count())
}

fn main() {
    let input = read_to_string("src/input/day06.txt").expect("Could not find input");
    let data = parse_input(&input).expect("Parsing must succeed");

    println!("The product of our victories is {}", part_one(&data));
    println!(
        "The number of chances at victory at the big race is {}",
        part_two(&data).expect("The race must succeed!")
    );
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

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day06-test.txt").expect("Could not find example");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert_eq!(part_two(&data), Ok(71503));
    }
}
