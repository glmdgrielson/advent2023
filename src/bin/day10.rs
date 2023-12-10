//! Day 10 of Advent of Code
//!
//! Pipe Maze
//! =========
//!
//! We are somewhere in a Metal Island,
//! full of twisty metal pipes, all alike.
//! There's a creature here with us, and
//! we want to know more.

use std::fs::read_to_string;

use simple_grid::{Grid, GridIndex};

use advent_2023::ParseError;

#[derive(Clone, Debug, PartialEq)]
struct Maze(Grid<Pipe>);

impl Maze {
    /// Find the index of the creature's starting position.
    ///
    /// # Panics
    ///
    /// If the grid doesn't HAVE the starting position.
    fn find_start(&self) -> GridIndex {
        let Some(idx) = self.0.position(|pipe| *pipe == Pipe::StartingPosition) else {
            unreachable!("Data should contain the creature's starting position");
        };

        idx
    }

    fn pipe_neighbors(&self, idx: GridIndex) -> Option<(GridIndex, GridIndex)> {
        let pipe = &self.0[idx];
        match pipe {
            // We know for a fact there are no neighbors.
            Pipe::Ground => None,
            // These are a lot of repetitive code
            // because we're repeating logic here.
            Pipe::NorthSouth => {
                // Sanity checks to see that the two
                // directions we're supposed to connect
                // to actually exist.
                let Some(north) = idx.up() else {
                    return None;
                };
                let Some(south) = idx.down() else {
                    return None;
                };

                // Get the neighboring pipes
                let north_p = &self.0[north];
                let south_p = &self.0[south];

                // Check that the pipes connect back to us.
                //
                // Note that if we connect to StartingPosition,
                // we may have None as our result, so we need
                // to check that we did not receive an explicit `false`.
                if north_p.connects_south() != Some(false)
                    && south_p.connects_north() != Some(false)
                {
                    Some((north, south))
                } else {
                    None
                }
            }
            Pipe::EastWest => {
                let Some(east) = idx.right() else {
                    return None;
                };
                let Some(west) = idx.left() else {
                    return None;
                };

                let east_p = &self.0[east];
                let west_p = &self.0[west];

                if east_p.connects_west() != Some(false) && west_p.connects_east() != Some(false) {
                    Some((east, west))
                } else {
                    None
                }
            }
            Pipe::NorthEast => {
                let Some(north) = idx.up() else {
                    return None;
                };
                let Some(east) = idx.right() else {
                    return None;
                };

                let north_p = &self.0[north];
                let east_p = &self.0[east];

                if north_p.connects_south() != Some(false) && east_p.connects_west() != Some(false)
                {
                    Some((north, east))
                } else {
                    None
                }
            }
            Pipe::NorthWest => {
                let Some(north) = idx.up() else {
                    return None;
                };
                let Some(west) = idx.left() else {
                    return None;
                };

                let north_p = &self.0[north];
                let west_p = &self.0[west];

                if north_p.connects_south() != Some(false) && west_p.connects_east() != Some(false)
                {
                    Some((north, west))
                } else {
                    None
                }
            }
            Pipe::SouthEast => {
                let Some(south) = idx.down() else {
                    return None;
                };
                let Some(east) = idx.right() else {
                    return None;
                };

                let south_p = &self.0[south];
                let east_p = &self.0[east];

                if south_p.connects_north() != Some(false) && east_p.connects_west() != Some(false)
                {
                    Some((south, east))
                } else {
                    None
                }
            }
            Pipe::SouthWest => {
                let Some(south) = idx.down() else {
                    return None;
                };
                let Some(west) = idx.left() else {
                    return None;
                };

                let south_p = &self.0[south];
                let west_p = &self.0[west];

                if south_p.connects_north() != Some(false) && west_p.connects_east() != Some(false)
                {
                    Some((south, west))
                } else {
                    None
                }
            }
            // This is the complicated one.
            Pipe::StartingPosition => {
                let mut neighbors = Vec::new();

                // We need to check each cardinal direction
                // to see if it connects to this space.
                //
                // We can check for just `Some(true)` because
                // this spot is the only one that could return `None`.
                if let Some(north) = idx.up() {
                    if self.0[north].connects_south() == Some(true) {
                        neighbors.push(north);
                    }
                }
                if let Some(south) = idx.down() {
                    if self.0[south].connects_north() == Some(true) {
                        neighbors.push(south);
                    }
                }
                if let Some(east) = idx.right() {
                    if self.0[east].connects_west() == Some(true) {
                        neighbors.push(east);
                    }
                }
                if let Some(west) = idx.left() {
                    if self.0[west].connects_east() == Some(true) {
                        neighbors.push(west);
                    }
                }

                // Check that exactly two directions connect here.
                assert_eq!(
                    neighbors.len(),
                    2,
                    "starting position should connect to only two pipes"
                );
                Some((neighbors[0], neighbors[1]))
            }
        }
    }

    fn find_loop(&self) -> Option<Vec<GridIndex>> {
        let start = self.find_start();
        let (next, _) = self.pipe_neighbors(start)?;

        let mut curr = next;
        let mut pipes = vec![start];
        loop {
            let (one, two) = self.pipe_neighbors(curr)?;
            let last = pipes.last()?;
            let next = if one == *last { two } else { one };
            pipes.push(curr);
            curr = next;
            if curr == start {
                break;
            }
        }

        Some(pipes)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Pipe {
    /// Cell does not contain a pipe.
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    /// Cell contains the animal.
    ///
    /// We do not know where this pipe connects _to._
    StartingPosition,
}

impl Pipe {
    /// Do we know if this pipe connects to the north?
    fn connects_north(&self) -> Option<bool> {
        match *self {
            Self::NorthSouth | Self::NorthEast | Self::NorthWest => Some(true),
            Self::Ground | Self::EastWest | Self::SouthEast | Self::SouthWest => Some(false),
            // We can't say for certain here, so return a dud value
            Self::StartingPosition => None,
        }
    }

    fn connects_south(&self) -> Option<bool> {
        match *self {
            Self::NorthSouth | Self::SouthEast | Self::SouthWest => Some(true),
            Self::Ground | Self::EastWest | Self::NorthEast | Self::NorthWest => Some(false),
            Self::StartingPosition => None,
        }
    }

    fn connects_east(&self) -> Option<bool> {
        match *self {
            Self::EastWest | Self::NorthEast | Self::SouthEast => Some(true),
            Self::Ground | Self::NorthSouth | Self::NorthWest | Self::SouthWest => Some(false),
            Self::StartingPosition => None,
        }
    }

    fn connects_west(&self) -> Option<bool> {
        match *self {
            Self::EastWest | Self::NorthWest | Self::SouthWest => Some(true),
            Self::Ground | Self::NorthSouth | Self::NorthEast | Self::SouthEast => Some(false),
            Self::StartingPosition => None,
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            'F' => Ok(Self::SouthEast),
            '7' => Ok(Self::SouthWest),
            'S' => Ok(Self::StartingPosition),
            _ => Err(ParseError::InvalidFormat("valid pipe character")),
        }
    }
}

/// Input consists of a grid of pipes.
///
/// Pipes can be:
/// - '.': Not actually a pipe.
/// - '|': A pipe going north-south.
/// - '-': A pipe going east-west.
/// - 'L': A pipe going north-east.
/// - 'J': A pipe going north-west.
/// - '7': A pipe going south-west.
/// - 'F': A pipe going south-east.
/// - 'S': A pipe that the animal has snuck in.
fn parse_input(input: &str) -> Result<Maze, ParseError> {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| Pipe::try_from(ch))
                .collect::<Result<Vec<Pipe>, ParseError>>()
        })
        .collect::<Result<Vec<Vec<Pipe>>, ParseError>>()?;

    let mut grid = Grid::new(0, 0, vec![]);
    for row in data {
        grid.push_row(row);
    }

    Ok(Maze(grid))
}

fn part_one(data: &Maze) -> Option<usize> {
    let pipes = data.find_loop()?;

    Some(pipes.len().div_ceil(2))
}

#[allow(unused)]
fn part_two(data: &Maze) {
    unimplemented!();
}

fn main() {
    let input = read_to_string("src/input/day10.txt").expect("Could not read input");
    let data = parse_input(&input).expect("Parsing should succeed");

    let one = part_one(&data).expect("Loop should exist");
    println!("Distance farthest from creature is {}", one);
}

#[cfg(test)]
mod test {
    // This is using the noisy version of the complex
    // loop as the example input as it gives a more
    // helpful set of test cases, I think.

    use super::*;

    #[test]
    fn test_parse_input() {
        use super::Pipe::*;

        let input = read_to_string("src/input/day10-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        let grid = data.0;
        assert_eq!(grid.dimensions(), (5, 5));

        let row = grid.row_iter(0).cloned().collect::<Vec<_>>();

        assert_eq!(
            row,
            vec![SouthWest, EastWest, SouthEast, SouthWest, EastWest],
            "Test uses noisy version of second input, check test data"
        );
    }

    #[test]
    fn test_find_start() {
        let input = read_to_string("src/input/day10-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert_eq!(data.find_start(), GridIndex::new(0, 2));
    }

    #[test]
    fn test_pipe_neighbors() {
        let input = read_to_string("src/input/day10-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        // This is a mostly randomly picked pipe index.
        let pipe = GridIndex::new(3, 3);
        assert_eq!(
            data.pipe_neighbors(pipe),
            Some((GridIndex::new(4, 3), GridIndex::new(2, 3)))
        );
        // Chosen as a ground space.
        let ground = GridIndex::new(0, 1);
        assert_eq!(data.pipe_neighbors(ground), None);
        let start = GridIndex::new(0, 2);
        assert_eq!(
            data.pipe_neighbors(start),
            Some((GridIndex::new(0, 3), GridIndex::new(1, 2)))
        );
    }

    #[test]
    fn test_find_loop() {
        let input = read_to_string("src/input/day10-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert!(data.find_loop().is_some_and(|pipes| pipes.len() == 16));
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day10-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert_eq!(part_one(&data), Some(8));
    }
}
