//! Day 07 of Advent of Code 2023
//!
//! Camel Cards
//! ===========
//!
//! We're playing poker today, folks!

use std::fs::read_to_string;

use advent_2023::ParseError;

#[derive(Clone, Debug)]
struct Hand {
    pub bid: u32,
    pub cards: [u32; 5],
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

// impl PartialOrd for Hand
// impl Ord for Hand

#[derive(Clone, Debug, PartialEq, Eq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

/// Input consists of a series of hands.
///
/// A hand consists of five cards and a bid,
/// where a card is one of 'A' (ace), 'K' (king),
/// 'Q' (queen), 'J' (jack), 'T' (ten), '9', '8',
/// '7', '6', '5', '4', '3', or '2'.
fn parse_input(input: &str) -> Result<Vec<Hand>, ParseError> {
    input
        .lines()
        .map(|line| {
            let Some((cards, bid)) = line.split_once(' ') else {
                return Err(ParseError::InvalidFormat("correct hand format"));
            };
            let cards = cards
                .chars()
                // This DOES mean that each of the numbers is one
                // off from the card face, but eh, it's a cost
                // I'm willing to live with at the moment.
                .map(|card| match card {
                    'A' => Ok(13),
                    'K' => Ok(12),
                    'Q' => Ok(11),
                    'J' => Ok(10),
                    'T' => Ok(9),
                    '9' => Ok(8),
                    '8' => Ok(7),
                    '7' => Ok(6),
                    '6' => Ok(5),
                    '5' => Ok(4),
                    '4' => Ok(3),
                    '3' => Ok(2),
                    '2' => Ok(1),
                    _ => Err(ParseError::InvalidFormat("valid card")),
                })
                .collect::<Result<Vec<u32>, ParseError>>()?;
            // Make sure cards is exactly five characters long.
            let Ok(cards) = <[u32; 5]>::try_from(cards) else {
                return Err(ParseError::InvalidFormat("proper hand length"));
            };

            let bid = bid.parse::<u32>()?;
            Ok(Hand { bid, cards })
        })
        .collect()
}

/// Part 1
/// ------
///
/// For a group of hands, calculate the score
/// by multiplying the bid by the rank. The
/// rank is calculated such that the weakest
/// hand is rank 1 and the strongest hand has
/// the rank equal to the number of elements
/// in the group. The answer is the sum of
/// all of the hands.
fn part_one(data: &[Hand]) -> u32 {
    unimplemented!();
}

#[allow(dead_code)]
fn part_two(data: &[Hand]) {
    unimplemented!("Part one incomplete!");
}

fn main() {
    let input = read_to_string("src/input/day07.txt").expect("Could not read input");
    let data = parse_input(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let example = "AT769 32";
        let actual = parse_input(example).unwrap();

        let expected = Hand {
            bid: 32,
            cards: [13, 9, 6, 5, 8],
        };

        assert_eq!(vec![expected], actual);
    }
}
