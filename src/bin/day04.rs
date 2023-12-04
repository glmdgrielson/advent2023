//! Day 04 of Advent of Code 2023
//!
//! Scratchcards
//! ============
//! We've met an elf to help us on our journey!
//! But right now they want us to look through their lottery tickets.
//! Sure, why not.

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

/// A ticket this elf was using.
#[derive(Clone, Debug, PartialEq)]
struct Ticket {
    /// The number associated with this card.
    ///
    /// This is a `usize` because it's going to be used
    /// as an index in part two, so it's convenient
    /// to make the change here, even though Rust
    /// defaults to making numbers `i`/`u32` most of the time.
    pub id: usize,
    /// The list of winning numbers for this ticket.
    pub expected: HashSet<u32>,
    /// The list of numbers the elf put down for this ticket.
    pub actual: HashSet<u32>,
}

/// Input consists of a series of tickets. A ticket has
/// an identifying number, a series of winning numbers,
/// and the series of numbers the elf actually tried to
/// win this ticket with.
///
/// ```notrust
/// Card N: A B C | W X Y Z
/// ```
fn parse_input(input: &str) -> Vec<Ticket> {
    input
        .lines()
        .map(|line| {
            // Remove the constant prefix.
            let line = line
                .strip_prefix("Card")
                .expect("Invalid ticket header")
                .trim_start();

            // Remove the card number from the string and store it.
            let (id, ticket) = line.split_once(": ").expect("Missing ticket data");
            // Convert the card number into an actual number.
            let id = id.parse().expect("Invalid ticket ID");

            // Split the winning numbers from the guessed numbers.
            let (expected, actual) = ticket.split_once(" | ").expect("Invalid ticket format");

            // Convert the various numbers into integers.
            let expected = expected
                // We have to use this function because there
                // isn't a constant separation between two
                // consecutive numbers in this puzzle,
                // which is a little annoying.
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let actual = actual
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            Ticket {
                id,
                expected,
                actual,
            }
        })
        .collect()
}

/// Part 1
/// ------
///
/// How well did our elf do? Victory is calculated as such:
/// - If the elf got one number correct, that's worth one point.
/// - For every number after the first on that ticket, double the score.
fn part_one(data: &[Ticket]) -> u32 {
    data.iter().fold(0, |acc, ticket| {
        // Find the numbers that won.
        let winners = ticket
            .expected
            .intersection(&ticket.actual)
            .collect::<Vec<_>>();

        // println!("Winners: {:?}", winners);

        let total = winners.len() as u32;

        // The explicit tag on the two is necessary to get the compiler
        // to shut up for some reason. I would love to know why.
        let score = if total >= 1 { 2_u32.pow(total - 1) } else { 0 };

        // println!("Card {}'s score is {}", ticket.id, score);

        acc + score
    })
}

/// Part 2
/// ------
///
/// Oh WHY. So it turns out that there's not really a score system
/// in place. Instead, winning gets you more scratch cards.
///
/// How many scratch cards does this horrid system give you
/// when you're done with all of this nonsense?
fn part_two(data: &[Ticket]) -> u32 {
    // Add the one ticket we know exists for each entry,
    // the one we already had to begin with.
    let mut ticket_count: HashMap<usize, u32> = data.iter().map(|t| (t.id, 1)).collect();

    data.iter().for_each(|ticket| {
        // Get a separate handle on the number of copies
        // of this ticket that we already have. We need
        // the `clone` in here to satisfy the borrow checker
        // since modifying a map that we're in the process
        // of using is only questionably safe, which isn't
        // good enough for the dang compiler.
        let this_count = ticket_count.get(&ticket.id).unwrap().clone();

        ticket
            .expected
            .intersection(&ticket.actual)
            // Get the number of winners
            .enumerate()
            // Convert winners to extra scratch off tickets.
            // Note that we add one so that we don't add
            // extra copies of THIS ticket.
            .map(|(idx, _)| ticket.id + idx + 1)
            // Update the total of cards for each winner.
            .for_each(|t| {
                // Find the ticket in the map.
                // We know we'll always find the ticket,
                // due to the constraints of the puzzle, but
                // Rust means I don't necessarily need to know that.
                ticket_count.entry(t).and_modify(|count| {
                    // Add more tickets to it.
                    //
                    // We specifically need to add one ticket
                    // for every copy of this particular
                    // ticket that we have. This was something
                    // I had to figure out the hard way.
                    *count += this_count;
                });
            });
    });

    ticket_count.values().fold(0, |acc, count| acc + count)
}

fn main() {
    let input = read_to_string("src/input/day04.txt").expect("Could not find input");
    let data = parse_input(&input);

    println!("The elf's total winnings today is {}", part_one(&data));
    println!(
        "The total number of Fibonacci brand scratch cards is {}",
        part_two(&data)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "Card 1: 3 6 9 | 2 7 8\n";

        let expected = Ticket {
            id: 1,
            expected: vec![3, 6, 9].into_iter().collect(),
            actual: vec![2, 7, 8].into_iter().collect(),
        };
        let actual = &parse_input(input)[0];

        assert_eq!(&expected, actual);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day04-test.txt").expect("Could not find example");
        let data = parse_input(&input);

        assert_eq!(part_one(&data), 13);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day04-test.txt").expect("Could not find example");
        let data = parse_input(&input);

        assert_eq!(part_two(&data), 30);
    }
}
