//! Day 2 of Advent of Code
//!
//! Cube Conundrum
//! --------------
//! An elf wants to play a game of cubes with us.

use std::fs::read_to_string;

/// The state of a particular game.
///
/// This exactly corresponds to one line
/// in the original input. AoC is nice like that.
/// Usually.
#[derive(Clone, Default, Debug, PartialEq)]
struct Game {
    /// The game ID
    pub id: usize,
    /// A list of rounds. It is assumed that cubes
    /// are replaced after each round, so one
    /// round has no impact on the next.
    pub rounds: Vec<Round>,
}

#[derive(Clone, Default, Debug, PartialEq)]
/// The data of one pull from the bag.
struct Round {
    /// The number of red cubes pulled this round
    pub red: usize,
    /// The number of blue cubes pulled this round
    pub blue: usize,
    /// The number of green cubes pulled this round
    pub green: usize,
}

/// Input consists of a set of games, with each game
/// consisting of a series of numbers of cubes.
///
/// ```notrust
/// Game X: R red, G green; B blue, R red; G green, B blue
/// ```
///
/// It is not guaranteed that this will happen a fixed
/// number of times or in a fixed order, nor will every
/// color show up every round. It is guaranteed that each
/// color will only show up once per pull however.
fn parse_input(data: &str) -> Vec<Game> {
    let mut vec = Vec::new();

    for line in data.lines() {
        // I don't need or want an iterator at this point
        // so I'm using `split_once` this time.
        let (id, data) = line.split_once(": ").expect("Invalid game");

        // The prefix is assumed by the setup of the puzzle.
        let id = id
            .strip_prefix("Game ")
            .expect("Missing prefix")
            // The structure of the puzzle also lets us
            // assume that this check will always succeed,
            // so we can unwrap here.
            .parse()
            .expect("Invalid ID");

        let mut game = Game {
            id,
            rounds: Vec::new(),
        };

        // Split the game by rounds
        for tally in data.split("; ") {
            let mut round: Round = Default::default();

            // Split the round into colors
            for pull in tally.split(", ") {
                // Separate number and color of cubes
                let (count, color) = pull.split_once(" ").expect("Invalid pull");

                // Turn the number of cubes into a number the computer recognizes
                let count = count.parse().expect("Invalid number of cubes");

                // Check to see if this number is greater than any
                // of the other pulls
                match color {
                    "red" => {
                        round.red = count;
                    }
                    "blue" => {
                        round.blue = count;
                    }
                    "green" => {
                        round.green = count;
                    }
                    _ => unreachable!("Impossible color of cube"),
                }
            }

            game.rounds.push(round);
        }

        // eprintln!("{:?}", game);

        vec.push(game);
    }

    vec
}

/// Part 1
/// ------
/// If we assume the bag the cubes are pulled from
/// only contains 12 red cubes, 13 green cubes,
/// and 14 blue cubes, how many of these games can
/// actually be physically possible?
///
/// We are assuming for this part that the cubes are
/// being put BACK into the bag, but that feels like
/// the sort of assumption part two will tell us is
/// false... (Pleasantly, I was quite wrong in this
/// assumption, and part 2 was actually dead simple.)
fn part_one(data: &[Game]) -> usize {
    let mut sum = 0;

    for game in data {
        let red_max = game
            .rounds
            .iter()
            .max_by_key(|r| r.red)
            .expect("Should have cubes")
            .red;
        if red_max > 12 {
            continue;
        }

        let green_max = game.rounds.iter().max_by_key(|r| r.green).unwrap().green;
        if green_max > 13 {
            continue;
        }

        let blue_max = game.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;
        if blue_max > 14 {
            continue;
        }

        sum += game.id;
    }

    sum
}

/// Part 2
/// ------
/// Now the elf is giving us a puzzle, specifically,
/// for each game, what is the fewest number of cubes
/// required for the game as described to be possible?
///
/// We are assuming for each game that every color shows
/// up at least once, so that the power (the calculation
/// we need to make at the end) doesn't somehow wind up
/// being zero.
fn part_two(data: &[Game]) -> usize {
    data.iter().fold(0, |acc, game| {
        let red_max = game.rounds.iter().max_by_key(|r| r.red).unwrap().red;
        let green_max = game.rounds.iter().max_by_key(|r| r.green).unwrap().green;
        let blue_max = game.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;

        let power = red_max * green_max * blue_max;

        acc + power
    })
}

fn main() {
    let input = read_to_string("src/input/day02.txt").expect("Could not read input data");
    let games = parse_input(&input);

    println!("Sum of valid games is {}", part_one(&games));

    println!("The total power of the games is {}", part_two(&games));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "Game 1: 3 blue, 1 red; 1 red, 2 green, 6 blue; 2 green";
        let actual = parse_input(input);

        let expected = vec![Game {
            id: 1,
            rounds: vec![
                Round {
                    blue: 3,
                    red: 1,
                    ..Default::default()
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Round {
                    green: 2,
                    ..Default::default()
                },
            ],
        }];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let data = read_to_string("src/input/day02-test.txt").unwrap();
        let example = parse_input(&data);

        assert_eq!(part_one(&example), 8);
    }

    #[test]
    fn test_part_two() {
        let data = read_to_string("src/input/day02-test.txt").unwrap();
        let games = parse_input(&data);

        assert_eq!(part_two(&games), 2286);
    }
}
