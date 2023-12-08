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
/// An enum to represent what could
/// probably be a boolean, but is
/// more descriptive in this format.
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
/// The map, as described by the input.
struct Map {
    /// The list of directions we are to take.
    pub directions: Vec<Direction>,
    /// A mapping of node names to the two paths
    /// that node connects to. The first element
    /// in the value tuple is the left path,
    /// and the second is the right path.
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
    // Map the direction characters to direction values.
    let directions = directions
        .chars()
        .map(|ch| match ch {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(ParseError::InvalidFormat("proper direction")),
        })
        // Check to see if there are errors and bail out if needed.
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

            // Convert the &str to String.
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

    // Since we may run out of directions,
    // we infinitely loop them here. This means
    // that the `break` is entirely vital, since
    // without it, this would loop forever.
    for direction in data.directions.iter().cycle() {
        // Check that there's a node here, otherwise fail.
        let Some(node) = data.network.get(curr) else {
            return None;
        };
        // Replace the current node with the one down the path.
        curr = match *direction {
            Direction::Left => &node.0,
            Direction::Right => &node.1,
        };
        // Increment the step counter
        // BEFORE we check the destination.
        steps += 1;
        // Are we there yet?
        if *curr == "ZZZ" {
            break;
        }
    }

    Some(steps)
}

/// Part 2
/// ------
///
/// So going from point AAA to point ZZZ
/// was a total bust. Let's try something
/// else: there's a one to one mapping of
/// '??A' nodes to '??Z' nodes. How long
/// until we get from all of the A nodes
/// to only Z nodes?
fn part_two(data: &Map) -> Option<u64> {
    // Get the list of A nodes.
    let curr = data
        .network
        .keys()
        .filter(|name| name.ends_with('A'))
        .collect::<Vec<_>>();

    // Right, so this is a BIG hack.
    // TODO: Find a solution that doesn't rely on unspoken assumptions.
    //
    // Every path from an A node to a Z node forms a cycle.
    // If we can find all such cycles, then the number
    // of steps the problem requires is just the least common
    // multiple of the lengths of each cycle.
    //
    // So we just need to find the cycles.
    let res = curr
        .iter()
        .map(|&name| {
            let mut curr = &name.clone();
            let mut steps = 0;

            // Again, we loop the directions infinitely
            // in case we run out midway through.
            for direction in data.directions.iter().cycle() {
                // Check that the node we're at actually exists.
                let Some(node) = data.network.get(curr) else {
                    return None;
                };
                // Replace the node we're at
                // with the node we're going to.
                curr = match *direction {
                    Direction::Left => &node.0,
                    Direction::Right => &node.1,
                };
                // Increment the step counter
                // BEFORE we consider ending the loop.
                steps += 1;
                // Check if we're at a Z node.
                if curr.ends_with('Z') {
                    break;
                }
            }

            Some(steps)
        })
        // Remove any elements that failed to navigate.
        .flatten()
        .collect::<Vec<u64>>();

    // Double check that every node navigated correctly.
    if res.len() != curr.len() {
        None
    } else {
        // Get the least common multiple of all of the cycles.
        //
        // This SHOULD be a call to `reduce`, but the lifetimes
        // were too much of a mess for me to want to put up with.
        Some(res.iter().fold(1, |one, two| lcm(one, *two)))
    }
}

/// Greatest common divisor
///
/// This is kind of a dumb but
/// simple way to go about it,
/// but it certainly works.
fn gcd(a: u64, b: u64) -> u64 {
    let mut one = a;
    let mut two = b;

    while two != 0 {
        let rem = one % two;
        one = two;
        two = rem;
    }

    one
}

/// Least common multiple
///
/// The multiplication here is why Part 2
/// returns a `u64`; it overflowed on `u32`.
fn lcm(one: u64, two: u64) -> u64 {
    (one * two) / gcd(one, two)
}

fn main() {
    let input = read_to_string("src/input/day08.txt").expect("Could not read data");
    let data = parse_input(&input).expect("Parsing failed");

    let steps = part_one(&data).expect("Network traversal failed");
    println!("Number of steps from AAA to ZZZ is {}", steps);

    println!(
        "Number of steps for ghost route is {}",
        part_two(&data).expect("Traversal should complete")
    );
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

    // No test for part two here because it would require another input file.
}
