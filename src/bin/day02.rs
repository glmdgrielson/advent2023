//! Day 2 of Advent of Code
//!
//! Cube Conundrum
//! --------------
//! An elf wants to play a game of cubes with us.

use std::fs::read_to_string;

/// The number of cubes per game.
#[derive(Clone, Default, Debug)]
struct Game {
    /// The game ID
    pub id: usize,
    /// The maximum number of red cubes
    pub red_max: usize,
    /// The maximum number of green cubes
    pub green_max: usize,
    /// The maximum number of blue cubes.
    pub blue_max: usize,
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
/// color show up every round.
fn parse_input(data: String) -> Vec<Game> {
    let mut vec = Vec::new();

    for line in data.lines() {
        // I don't need or want an iterator at this point
        // so I'm using `split_once` this time.
        let (id, data) = line.split_once(": ").expect("Invalid game");

        // The prefix is assumed by the setup of the puzzle.
        let id = id
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .expect("Invalid ID");

        let mut game = Game {
            id,
            ..Default::default()
        };

        // Split the game by rounds
        for round in data.split("; ") {
            // Split the round into colors
            for pull in round.split(", ") {
                // Separate number and color of cubes
                let (count, color) = pull.split_once(" ").expect("Invalid pull");

                // Turn the number of cubes into a number the computer recognizes
                let count = count.parse().expect("Invalid number of cubes");

                // Check to see if this number is greater than any
                // of the other pulls
                match color {
                    "red" => {
                        if count > game.red_max {
                            game.red_max = count;
                        }
                    }
                    "blue" => {
                        if count > game.blue_max {
                            game.blue_max = count;
                        }
                    }
                    "green" => {
                        if count > game.green_max {
                            game.green_max = count;
                        }
                    }
                    _ => unreachable!("Impossible color of cube"),
                }
            }
        }

        vec.push(game);
    }

    vec
}

fn part_one() {}

fn part_two() {
    unimplemented!("Part one incomplete!")
}

fn main() {
    let input = read_to_string("input/day02.txt").expect("Could not read input data");
    let games = parse_input(input);
}

#[cfg(test)]
mod test {}
