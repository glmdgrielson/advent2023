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
/// A sequence provided in the OASIS report.
struct History(Vec<i32>);

impl History {
    /// Find the next item in this sequence.
    fn next(&self) -> i32 {
        // Check that we don't have an empty sequence.
        //
        // We don't know the length of the sequence
        // off the top of our heads, and Rust only
        // gives us the last element if the sequence
        // isn't empty, so this is mainly for waving
        // off a pathological edge case. Thanks, Rust!
        let Some(&last) = self.0.last() else {
            unreachable!("Invalid input received");
        };

        // We need to collect all of the final
        // elements we get from this process.
        // Start with the last element of the
        // sequence we were originally given.
        let mut last = vec![last];
        // This sequence contains the current
        // stage of the reduction process.
        // Every time we do the process, we come
        // out with one element less. We can
        // only hope that we get to all zeroes
        // before we get to only one element.
        let mut diff = self.0.clone();
        loop {
            // Check if we've done all of the
            // differentiation we can.
            if diff.iter().all(|&val| val == 0) {
                break;
            }

            diff = diff
                // Get pairs of consecutive elements.
                .windows(2)
                // Get the difference of each pair.
                .map(|window| window[1] - window[0])
                // Return this to Vec so we can
                // assign it to our "accumulator".
                .collect();

            // Pathological safety check to see that
            // we didn't wind up with an empty sequence.
            let Some(&new_last) = diff.last() else {
                unreachable!("History should not reduce to empty");
            };

            // Add the last item of this stage
            // to the list to sum up later.
            last.push(new_last);
        }

        // Sum up the last value
        // of every stage that
        // we have processed.
        last.iter().sum()
    }

    /// Find the hypothetical previous number in this sequence.
    ///
    /// So I tried two implementations of this, both of which
    /// passed the test on the sample input. The one commented
    /// out calculates the values by hand, using a similar
    /// logic to the `next` function. For some reason, this
    /// gives me an answer that is way off and NEGATIVE,
    /// which AoC doesn't seem suited to handle oddly enough.
    ///
    /// The solution that worked reversed the sequence
    /// in this history and then used `next` directly. That
    /// gave the solution AoC was looking for.
    fn prev(&self) -> i32 {
        // TODO: Find out why this solution didn't work.

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

        // Get the data inside of this struct.
        let mut data = self.0.clone();
        // Reverse the data so that the
        // first element is at the end.
        data.reverse();

        // Find the "next" element
        // of the reversed sequence.
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
                // Some values are NEGATIVE in this input,
                // so we're using an `i32`, not a `u32`.
                .map(|val| val.parse::<i32>().map_err(ParseError::ExpectedNumber))
                // Check to see if we have any errors.
                .collect::<Result<Vec<i32>, _>>()?;
            // If not, pass off the sequence.
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
