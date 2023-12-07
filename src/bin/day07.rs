//! Day 07 of Advent of Code 2023
//!
//! Camel Cards
//! ===========
//!
//! We're playing poker today, folks!

use std::collections::HashSet;
use std::fs::read_to_string;

use advent_2023::ParseError;

/// The special card that needs unique treatment.
/// This corresponds to a Jack otherwise.
const JOKER: u32 = 10;

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

    fn joker_hand_type(&self) -> HandType {
        let (jokers, cards): (Vec<_>, Vec<u32>) = self.cards.iter().partition(|&card| *card == 0);

        // eprintln!("Jokers: {:?}, Cards: {:?}", jokers, cards);
        // I'm not sure why `into_iter` works while `iter` doesn't.
        let card_types = cards.into_iter().collect::<HashSet<u32>>();
        let mut counts: Vec<_> = card_types
            .iter()
            .map(|kind| self.cards.iter().filter(|&card| card == kind).count())
            .collect();
        // Sort by frequency
        counts.sort();
        // Get the highest count first.
        counts.reverse();

        // If EVERY card is a joker, continuing
        // will panic, so bail out here instead.
        if jokers.len() == 5 {
            return HandType::FiveOfKind;
        }
        let jokers = jokers.len();
        // counts[0] += jokers.len();

        match counts[0] {
            5 => HandType::FiveOfKind,
            4 => {
                if jokers == 1 {
                    HandType::FiveOfKind
                } else {
                    HandType::FourOfKind
                }
            }
            3 => {
                // If every other card is a joker,
                // the match below is going to panic,
                // so we need to check for it here.
                if jokers == 2 {
                    HandType::FiveOfKind
                } else {
                    // Check what comes next,
                    // because this could be a full
                    // house or a three of a kind.
                    match counts[1] {
                        // All cards are accounted for.
                        2 => HandType::FullHouse,
                        1 => {
                            // Three of one, one of another,
                            // check to see if we have a joker.
                            if jokers == 1 {
                                HandType::FourOfKind
                            } else {
                                HandType::ThreeOfKind
                            }
                        }
                        _ => unreachable!("Invalid hand"),
                    }
                }
            }
            // If we have a pair, check to find another one.
            2 => {
                // If every other card is a joker,
                // that match is going to panic,
                if jokers == 3 {
                    HandType::FiveOfKind
                } else {
                    match counts[1] {
                        2 => {
                            // Two pairs, which is a full
                            // house if we have a joker!
                            if jokers == 1 {
                                HandType::FullHouse
                            } else {
                                HandType::TwoPair
                            }
                        }
                        // Three cards accounted for,
                        // what about the other two?
                        1 => match jokers {
                            // Two, one, and two jokers
                            2 => HandType::FourOfKind,
                            // Two, one, a joker, and another one
                            1 => HandType::ThreeOfKind,
                            // Two match and every other
                            // card is unique and not a joker
                            0 => HandType::OnePair,
                            _ => unreachable!("Invalid hand"),
                        },
                        _ => unreachable!("Invalid hand"),
                    }
                }
            }
            // If we have a one here,
            // then every card must be unique,
            // or a joker, so match on joker count.
            1 => match jokers {
                4 => HandType::FiveOfKind,
                3 => HandType::FourOfKind,
                2 => HandType::ThreeOfKind,
                1 => HandType::OnePair,
                0 => HandType::HighCard,
                _ => unreachable!("Invalid hand"),
            },
            // EVERY card is a joker,
            // which is technically
            // five of a kind
            0 => HandType::FiveOfKind,
            _ => unreachable!("Invalid hand"),
        }
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

/// Part 2
/// ------
///
/// This is ALMOST the same question as part 1,
/// but with a catch: all of the jacks are now
/// jokers, which mean that they contribute to
/// whatever value makes the most points. The
/// question is the same: what's our score?
fn part_two(data: &[Hand]) -> usize {
    let mut res = data.to_vec();

    // Turn the Jokers into zeros.
    res = res
        .iter()
        .map(|hand| {
            let cards = hand
                .cards
                .iter()
                .map(|&card| if card == JOKER { 0 } else { card })
                .collect::<Vec<_>>();

            let cards = <[u32; 5]>::try_from(cards).expect("Operation should be infalliable");

            Hand {
                bid: hand.bid,
                cards,
            }
        })
        .collect();

    // Sort the hands as described by Part 2.
    res.sort_by(|one, two| {
        one.joker_hand_type()
            .cmp(&two.joker_hand_type())
            .then(two.cards.cmp(&one.cards))
    });
    // Reverse the cards.
    res.reverse();

    res.iter().enumerate().for_each(|(idx, hand)| {
        eprintln!("Hand {} is {:?}", idx, hand.cards);
    });
    // Sum the ranks
    res.iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * (hand.bid as usize))
        .sum()
}

fn main() {
    let input = read_to_string("src/input/day07.txt").expect("Could not read input");
    let data = parse_input(&input).expect("Parsing failed");

    println!("Total winnings are {}", part_one(&data));
    println!("Total winnings with jokers are {}", part_two(&data));
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

    #[test]
    fn test_part_two() {
        let example = read_to_string("src/input/day07-test.txt").expect("Could not read example");
        let data = parse_input(&example).expect("Data failed to parse");

        assert_eq!(part_two(&data), 5905);
    }

    const EDGE_CASES: &'static str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

    #[test]
    fn test_part_two_edge_cases() {
        let data = parse_input(EDGE_CASES).expect("Edge cases failed to parse");

        assert_eq!(part_two(&data), 6839);
    }
}
