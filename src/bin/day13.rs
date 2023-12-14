//! Day 13 of Advent of Code
//!
//! Point of Incidence
//! ==================
//!
//! Lava Island needs more lava.
//! Unfortunately, mirrors are
//! rather in the way.

use std::collections::VecDeque as Deque;
use std::fs::read_to_string;

use simple_grid::Grid;

use advent_2023::{ParseError, ParseResult};

#[derive(Clone, Debug, PartialEq)]
struct Frame(Grid<bool>);

impl Frame {
    /// Get the point where this frame mirrors.
    fn mirror_row(&self) -> Option<usize> {
        let deque = self
            .0
            .rows()
            .map(|row| self.0.row_iter(row).collect::<Vec<_>>())
            .collect::<Deque<_>>();
        (1..deque.len()).find(|&offset| {
            let one = deque.iter().take(offset).rev();
            let two = deque.iter().skip(offset);
            // eprintln!(
            //     "Left: {:?}\nRight: {:?}",
            //     one.collect::<Vec<_>>(),
            //     two.collect::<Vec<_>>()
            // );
            one.zip(two).all(|(row_one, row_two)| *row_one == *row_two)
        })
    }

    fn mirror_column(&self) -> Option<usize> {
        let deque = self
            .0
            .columns()
            .map(|row| self.0.column_iter(row).collect::<Vec<_>>())
            .collect::<Deque<_>>();
        (1..deque.len()).find(|&offset| {
            let one = deque.iter().take(offset).rev();
            let two = deque.iter().skip(offset);
            // eprintln!(
            //     "Left: {:?}\nRight: {:?}",
            //     one.collect::<Vec<_>>(),
            //     two.collect::<Vec<_>>()
            // );
            one.zip(two).all(|(row_one, row_two)| *row_one == *row_two)
        })
    }
}

/// Input consists of a series
/// of grids, where every position
/// on the grid is `.` or `#`.
fn parse_input(input: &str) -> ParseResult<Vec<Frame>> {
    input
        .split("\n\n")
        .map(|grid| {
            let mut map = Grid::new(0, 0, Vec::new());
            for line in grid.lines() {
                let vec = line
                    .chars()
                    .map(|ch| match ch {
                        '#' => Ok(true),
                        '.' => Ok(false),
                        _ => Err(ParseError::InvalidFormat("valid pattern character")),
                    })
                    .collect::<ParseResult<Vec<bool>>>()?;
                map.push_row(vec);
            }
            Ok(Frame(map))
        })
        .collect::<ParseResult<Vec<_>>>()
}

/// Part 1
/// ------
///
/// Find all of the reflection points
/// for each pattern, which could be
/// horizontal or vertical.
fn part_one(data: &[Frame]) -> usize {
    let columns = data
        .iter()
        .flat_map(|frame| frame.mirror_column())
        .sum::<usize>();
    let rows = data
        .iter()
        .flat_map(|frame| frame.mirror_row())
        .map(|row| row * 100)
        .sum::<usize>();

    columns + rows
}

#[allow(unused)]
fn part_two(data: &[Frame]) {
    unimplemented!();
}

fn main() {
    let input = read_to_string("src/input/day13.txt").expect("Could not read file");
    let data = parse_input(&input).expect("Parsing should succeed");

    println!("The mirror sum is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day13-test.txt").expect("Could not read file");
        let data = parse_input(&input).expect("Parsing failed");

        let expected = vec![true, false, true, true, false, false, true, true, false];
        let actual: Vec<bool> = data[0].0.row_iter(0).map(|cell| *cell).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mirror_position() {
        let input = read_to_string("src/input/day13-test.txt").expect("Could not read file");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(data[0].mirror_column(), Some(5));
        assert_eq!(data[0].mirror_row(), None);

        assert_eq!(data[1].mirror_row(), Some(4));
        assert_eq!(data[1].mirror_column(), None);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day13-test.txt").expect("Could not read file");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_one(&data), 405);
    }
}
