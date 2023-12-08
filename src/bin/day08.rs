//! Day 08 of Advent of Code
//!
//! Haunted Wasteland
//! =================
//!
//! We're stuck in a sandstorm, our guide has
//! randomly disappeared, and we are completely
//! lost. Our only hope comes in the form of...
//! a linked list. WONDERFUL.

use std::collections::HashMap;
use std::fs::read_to_string;

use advent_2023::ParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct Map {
    pub directions: Vec<Direction>,
    pub network: HashMap<String, (String, String)>,
}

/// Input consists of a series of left and right instructions
/// as well as a list of nodes in a network. Each node consists
/// of a name and two paths, a left and a right path. Per the
/// requirements of the puzzle, one node is named
/// "AAA" and one node is named "ZZZ".
fn parse_input(input: &str) -> Result<Map, ParseError> {
    // Check that we have two bits of input.
    let Some((directions, network)) = input.split_once("\n\n") else {
        return Err(ParseError::InvalidFormat("two sections of input"));
    };
    let directions = directions
        .chars()
        .map(|ch| match ch {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(ParseError::InvalidFormat("proper direction")),
        })
        .collect::<Result<Vec<Direction>, ParseError>>()?;

    let network = network
        .lines()
        .map(|line| {
            // Split the line into the name of the node
            // and its two pathways.
            let Some((name, paths)) = line.split_once(" = ") else {
                return Err(ParseError::InvalidFormat("'=' in node"));
            };

            // Remove the separators so that we're left with plain node names.
            let Some(paths) = paths.strip_prefix('(') else {
                return Err(ParseError::InvalidFormat("'(' in paths"));
            };
            let Some(paths) = paths.strip_suffix(')') else {
                return Err(ParseError::InvalidFormat("')' in paths"));
            };
            let Some((left, right)) = paths.split_once(", ") else {
                return Err(ParseError::InvalidFormat("comma separated paths"));
            };

            Ok((name.to_owned(), (left.to_owned(), right.to_owned())))
        })
        .collect::<Result<HashMap<String, (String, String)>, ParseError>>()?;

    Ok(Map {
        directions,
        network,
    })
}

/// Part 1
/// ------
///
/// Given the set of directions we were given,
/// how many steps does it take to get from
/// point "AAA" to point "ZZZ"?
///
/// Returns `None` if the network traversal
/// encounters an error.
fn part_one(data: &Map) -> Option<usize> {
    // The current node.
    let mut curr = &String::from("AAA");
    // How many steps we've taken.
    let mut steps = 0;

    for direction in data.directions.iter().cycle() {
        let Some(node) = data.network.get(curr) else {
            return None;
        };
        curr = match *direction {
            Direction::Left => &node.0,
            Direction::Right => &node.1,
        };
        steps += 1;
        if *curr == String::from("ZZZ") {
            break;
        }
    }

    Some(steps)
}

#[allow(unused)]
fn part_two(data: &Map) {
    unimplemented!("Part one incomplete");
}

fn main() {
    let input = read_to_string("src/input/day08.txt").expect("Could not read data");
    let data = parse_input(&input).expect("Parsing failed");

    let steps = part_one(&data).expect("Network traversal failed");
    println!("Number of steps from AAA to ZZZ is {}", steps);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day08-test.txt").expect("Could not read data");
        let data = parse_input(&input).expect("Parsing did not succeed");

        // Test direction parsing.
        assert_eq!(
            data.directions,
            vec![Direction::Left, Direction::Left, Direction::Right],
            "This uses the second example, check the test data"
        );

        // Test node parsing.
        let node = data.network.get("AAA").expect("AAA not present in network");
        assert_eq!(
            node,
            &("BBB".to_string(), "BBB".to_string()),
            "This uses the second example, check the test data"
        );
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day08-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(
            part_one(&data),
            Some(6),
            "This uses the second example, check the test data"
        );
    }
}
