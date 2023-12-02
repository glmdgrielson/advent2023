//! Day 2 of Advent of Code
//!
//! Cube Conundrum
//! --------------
//! An elf wants to play a game of cubes with us.

use std::fs::read_to_string;

/// The number of cubes per game.
#[derive(Clone, Default, Debug, PartialEq)]
struct Game {
    /// The game ID
    pub id: usize,
    /// A list of rounds
    pub rounds: Vec<Round>,
}

#[derive(Clone, Default, Debug, PartialEq)]
struct Round {
    pub red: usize,
    pub blue: usize,
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
/// color will only show up once.
fn parse_input(data: &str) -> Vec<Game> {
    let mut vec = Vec::new();

    for line in data.lines() {
        // I don't need or want an iterator at this point
        // so I'm using `split_once` this time.
        let (id, data) = line.split_once(": ").expect("Invalid game");

        // The prefix is assumed by the setup of the puzzle.
        let id = id
            .strip_prefix("Game ")
            .unwrap()
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
/// false...
fn part_one(data: &[Game]) -> usize {
    let mut sum = 0;

    for game in data {

        let red_max = game.rounds.iter().max_by_key(|r| r.red).expect("Should have cubes").red;
        if red_max > 12 {
            continue;
        }

        let blue_max = game.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;
        if blue_max > 13 {
            continue;
        }

        let green_max = game.rounds.iter().max_by_key(|r| r.green).unwrap().green;
        if green_max > 14 {
            continue;
        }

        sum += game.id;
    }

    sum
}

fn part_two() {
    unimplemented!("Part one incomplete!")
}

fn main() {
    let input = read_to_string("input/day02.txt").expect("Could not read input data");
    let games = parse_input(&input);

    println!("Sum of valid games is {}", part_one(&games));
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_parse_input() {
    //     let example = vec![
    //         Game {
    //             id: 1,
    //             red_max: 4,
    //             green_max: 2,
    //             blue_max: 6,
    //         },
    //         Game {
    //             id: 2,
    //             red_max: 1,
    //             green_max: 3,
    //             blue_max: 4,
    //         },
    //         Game {
    //             id: 3,
    //             red_max: 20,
    //             green_max: 13,
    //             blue_max: 6,
    //         },
    //         Game {
    //             id: 4,
    //             red_max: 14,
    //             green_max: 3,
    //             blue_max: 15,
    //         },
    //         Game {
    //             id: 5,
    //             red_max: 6,
    //             green_max: 3,
    //             blue_max: 2,
    //         },
    //     ];
    //     let data = read_to_string("src/input/day02-test.txt").expect("Could not read test data");
    //
    //     assert_eq!(example, parse_input(&data));
    // }

    #[test]
    fn test_part_one() {
        let data = read_to_string("src/input/day02-test.txt").unwrap();
        let example = parse_input(&data);

        assert_eq!(part_one(&example), 8);
    }
}
