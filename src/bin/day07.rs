//! Day 07 of Advent of Code 2023
//!
//! Camel Cards
//! ===========
//!
//! We're playing poker today, folks!

use std::collections::HashSet;
use std::fs::read_to_string;

use advent_2023::ParseError;

#[derive(Clone, Debug, Eq)]
struct Hand {
    pub bid: u32,
    pub cards: [u32; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let card_types = HashSet::from(self.cards);
        let mut counts: Vec<_> = card_types
            .iter()
            .map(|kind| self.cards.iter().filter(|&card| card == kind).count())
            .collect();
        // Sort by frequency
        counts.sort();
        // Get the highest count first.
        counts.reverse();
        // println!("Card counts for hand {:?} are {:?}", self.cards, counts);
        let kind = match counts[0] {
            5 => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            // Check what comes next,
            // because this could be a full
            // house or a three of a kind.
            3 => match counts[1] {
                2 => HandType::FullHouse,
                1 => HandType::ThreeOfKind,
                _ => unreachable!("Invalid hand"),
            },
            // If we have a pair, check to find another one.
            2 => match counts[1] {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => unreachable!("Invalid hand"),
            },
            // If we have a one here,
            // then every card must be unique.
            1 => HandType::HighCard,
            _ => unreachable!("Invalid hand"),
        };

        // println!("Cards {:?} have hand {:?}", self.cards, kind);

        kind
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(other.cards.cmp(&self.cards))
    }
}

// impl PartialOrd for Hand
// impl Ord for Hand

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// The types of hand a card can have.
///
/// Note that this does not store any
/// information about the hand itself,
/// because it's not actually very USEFUL.
enum HandType {
    // Five of a kind
    FiveOfKind,
    // Four of a kind
    FourOfKind,
    // Three of one, two of another
    FullHouse,
    // Three of a kind
    ThreeOfKind,
    // Two of one, two of another
    TwoPair,
    // Two of a kind
    OnePair,
    // Five different cards
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
                eprintln!("Line failed: {}", line);
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
fn part_one(data: &[Hand]) -> usize {
    let mut res = data.to_vec();
    // Sort the hands.
    res.sort();
    // Make the first hand the weakest.
    res.reverse();
    // res.iter().enumerate().for_each(|(idx, hand)| {
    //     println!("Hand {} is {:?}", idx, hand.cards);
    // });
    res.iter()
        // Get the rank of each hand...
        .enumerate()
        // ...and multiply it by the bid.
        .map(|(idx, hand)| (idx + 1) * (hand.bid as usize))
        .sum()
}

#[allow(dead_code)]
fn part_two(data: &[Hand]) {
    unimplemented!("Part one incomplete!");
}

fn main() {
    let input = read_to_string("src/input/day07.txt").expect("Could not read input");
    let data = parse_input(&input).expect("Parsing failed");

    println!("Total winnings are {}", part_one(&data));
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

    #[test]
    fn test_part_one() {
        let example = read_to_string("src/input/day07-test.txt").expect("Could not read example");
        let data = parse_input(&example).expect("Data failed to parse");

        assert_eq!(part_one(&data), 6440);
    }
}
