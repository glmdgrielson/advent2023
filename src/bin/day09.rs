//! Day 09 of Advent of Code 2023
//!
//! Mirage Maintenance
//! ==================
//!
//! Oh we're doing Sierpinksi triangle
//! nonsense... And basic calculus!

use std::fs::read_to_string;

use advent_2023::ParseError;

#[derive(Clone, Debug, PartialEq)]
struct History(Vec<i32>);

impl History {
    fn next(&self) -> i32 {
        let Some(&last) = self.0.last() else {
            unreachable!("Invalid input received");
        };

        let mut last = vec![last];
        let mut diff = self.0.clone();
        loop {
            if diff.iter().all(|&val| val == 0) {
                break;
            }

            diff = diff
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();

            let Some(&new_last) = diff.last() else {
                unreachable!("History should not reduce to empty");
            };
            last.push(new_last);
        }
        last.iter().sum()
    }

    fn prev(&self) -> i32 {
        // let Some(&head) = self.0.first() else {
        //     unreachable!("Invalid input received");
        // };
        //
        // let mut heads = vec![head];
        // let mut diff = self.0.clone();
        // loop {
        //     if diff.iter().all(|&val| val == 0) {
        //         break;
        //     }
        //
        //     diff = diff
        //         .windows(2)
        //         .map(|window| window[1] - window[0])
        //         .collect();
        //
        //     let Some(&new_head) = diff.first() else {
        //         unreachable!("History should not reduce to empty");
        //     };
        //     heads.push(new_head);
        // }
        //
        // heads.iter().fold(0, |acc, val| val - acc)
        let mut data = self.0.clone();
        data.reverse();

        History(data).next()
    }
}

/// Input consists of a series of sequences,
/// each sequence consists of numbers.
/// This should be fairly easy to parse.
fn parse_input(input: &str) -> Result<Vec<History>, ParseError> {
    input
        .lines()
        .map(|line| {
            let history = line
                .split_whitespace()
                .map(|val| val.parse::<i32>().map_err(ParseError::ExpectedNumber))
                .collect::<Result<Vec<i32>, _>>()?;
            Ok(History(history))
        })
        .collect()
}

/// Part 1
/// ------
///
/// For each sequence in the report, find
/// out what the next item should be. It's
/// gonna take a bit of calculus, since we
/// need to figure out the rate at which
/// the sequences grow.
///
/// When we've found out what all of the
/// sequences should be, we find the sum.
fn part_one(data: &[History]) -> i32 {
    data.iter().map(|history| history.next()).sum()
}

/// Part 2
/// ------
///
/// Well, since part one was so easy, surely
/// it's safe to apply the same logic backwards
/// right? ...right? Why do I hear a crying
/// engineer in the distance?
///
/// Find the sum of the hypothetical zeroth
/// entry in each of the provided sequences.
fn part_two(data: &[History]) -> i32 {
    data.iter().map(|history| history.prev()).sum()
}

fn main() {
    let input = read_to_string("src/input/day09.txt").expect("Could not read data");
    let data = parse_input(&input).expect("Parsing should succeed");

    println!("Sum of next steps is {}", part_one(&data));
    println!("Sum of hypothetical previous steps is {}", part_two(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day09-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Example should parse successfully");

        assert_eq!(data[0], History(vec![0, 3, 6, 9, 12, 15]));
    }

    #[test]
    fn test_history_next() {
        let input = read_to_string("src/input/day09-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Example should parse successfully");

        assert_eq!(data[0].next(), 18);
        assert_eq!(data[1].next(), 28);
    }

    #[test]
    fn test_history_prev() {
        let input = read_to_string("src/input/day09-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Example should parse successfully");

        assert_eq!(data[0].prev(), -3);
        assert_eq!(data[1].prev(), 0);
        assert_eq!(data[2].prev(), 5);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day09-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Example should parse successfully");

        assert_eq!(part_two(&data), 2);
    }
}
