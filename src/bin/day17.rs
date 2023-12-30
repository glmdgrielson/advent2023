//! Day 17 of Advent of Code 2023
//!
//! Clumsy Crucible
//! ===============
//!
//! Crucibles are attempting to transfer
//! heat, unfortunately the process is
//! extremely lossy.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

use simple_grid::{Grid, GridIndex};

use advent_2023::{Direction, ParseError, ParseResult};

/// Input consists of a grid of numbers.
fn parse_input(input: &str) -> ParseResult<Grid<u32>> {
    let mut grid = Grid::new(0, 0, vec![]);
    for line in input.lines() {
        let row = line
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .ok_or(ParseError::InvalidFormat("valid digit"))
            })
            .collect::<ParseResult<Vec<u32>>>()?;

        grid.push_row(row);
    }

    Ok(grid)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Crucible {
    cost: u32,
    location: GridIndex,
    direction: Direction,
    /// How many steps we've taken in the current direction.
    ///
    /// Can never be more than three.
    steps: u8,
}

impl Crucible {
    fn next_directions(&self) -> Vec<Direction> {
        match self.direction {
            Direction::North => vec![Direction::North, Direction::East, Direction::West],
            Direction::South => vec![Direction::South, Direction::East, Direction::West],
            Direction::East => vec![Direction::North, Direction::South, Direction::East],
            Direction::West => vec![Direction::North, Direction::South, Direction::West],
        }
    }

    fn next_steps(&self, costs: &Grid<u32>) -> Vec<Self> {
        self.next_directions()
            .iter()
            .filter_map(|&dir| {
                if dir == self.direction && self.steps == 3 {
                    None
                } else if let Some(location) = self.forward(costs) {
                    let cost = self.cost + costs[location];

                    let steps = if self.direction == dir {
                        self.steps + 1
                    } else {
                        1
                    };

                    Some(Crucible {
                        cost,
                        location,
                        direction: dir,
                        steps,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn forward(&self, costs: &Grid<u32>) -> Option<GridIndex> {
        match self.direction {
            Direction::North => costs.up_index(self.location),
            Direction::South => costs.down_index(self.location),
            Direction::East => costs.right_index(self.location),
            Direction::West => costs.left_index(self.location),
        }
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

/// Part 1
/// ------
///
/// What is the minimum amount of
/// heat loss possible, assuming we're
/// going from the northwest to the
/// southwest of the grid?
fn part_one(data: &Grid<u32>) -> u32 {
    let endpoint = GridIndex::new(data.width() - 1, data.height() - 1);

    let mut queue = BinaryHeap::from([
        Reverse(Crucible {
            cost: 0,
            location: GridIndex::new(1, 0),
            direction: Direction::South,
            steps: 1,
        }),
        Reverse(Crucible {
            cost: 0,
            location: GridIndex::new(0, 1),
            direction: Direction::East,
            steps: 1,
        }),
    ]);

    let mut seen = HashSet::new();

    // let mut target = Vec::new();

    while let Some(Reverse(crucible)) = queue.pop() {
        // println!(
        //     "Current position: {} heading {:?}",
        //     crucible.location, crucible.direction
        // );
        if crucible.location == endpoint {
            return crucible.cost;
        }

        for next in crucible.next_steps(data) {
            if seen.insert((next.location, next.direction, next.steps)) {
                queue.push(Reverse(next));
            }
        }
    }

    unreachable!("Endpoint was not reached!")
    // target.into_iter().min().expect("Target must be reached")
    // not 1015...
}

fn main() {
    let input = read_to_string("src/input/day17.txt").expect("Could not read file");
    let data = parse_input(&input).expect("Parsing failed");

    println!("Minimum heat loss for crucible is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day17-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        let expected = vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3];
        let actual: Vec<_> = data.row_iter(0).map(|num| *num).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day17-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_one(&data), 102);
    }
}
