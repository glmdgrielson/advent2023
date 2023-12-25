//! Day 16 of Advent of Code
//!
//! The Floor Will Be Lava
//! ======================
//!
//! We need to start up the lava
//! systems and that's gonna
//! involve lasers. FUN!

use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

use simple_grid::{Grid, GridIndex};

use advent_2023::{ParseError, ParseResult};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Beam {
    pub direction: Direction,
    pub position: GridIndex,
}

impl Beam {
    fn advance<T>(&self, data: &Grid<T>) -> Option<Self> {
        match self.direction {
            Direction::North => data.up_index(self.position).map(|position| Beam {
                direction: Direction::North,
                position,
            }),
            Direction::South => data.down_index(self.position).map(|position| Beam {
                direction: Direction::South,
                position,
            }),
            Direction::East => data.right_index(self.position).map(|position| Beam {
                direction: Direction::East,
                position,
            }),
            Direction::West => data.left_index(self.position).map(|position| Beam {
                direction: Direction::West,
                position,
            }),
        }
    }

    fn rotate_forward(&self) -> Self {
        match self.direction {
            Direction::North => Beam {
                direction: Direction::East,
                position: self.position,
            },
            Direction::South => Beam {
                direction: Direction::West,
                position: self.position,
            },
            Direction::East => Beam {
                direction: Direction::North,
                position: self.position,
            },
            Direction::West => Beam {
                direction: Direction::South,
                position: self.position,
            },
        }
    }

    fn rotate_back(&self) -> Self {
        match self.direction {
            Direction::North => Beam {
                direction: Direction::West,
                position: self.position,
            },
            Direction::South => Beam {
                direction: Direction::East,
                position: self.position,
            },
            Direction::East => Beam {
                direction: Direction::South,
                position: self.position,
            },
            Direction::West => Beam {
                direction: Direction::North,
                position: self.position,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Point {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplit,
    HorizontalSplit,
}

/// Input consists of a grid containing
/// one of the following characters:
/// - `.`: empty space
/// - `/` or `\\`: a mirror
/// - `|` or `-`: a splitter
fn parse_input(input: &str) -> ParseResult<Grid<Point>> {
    let mut grid: Grid<Point> = Grid::new(0, 0, Vec::new());

    for line in input.lines() {
        let row = line
            .chars()
            .map(|ch| match ch {
                '.' => Ok(Point::Empty),
                '/' => Ok(Point::ForwardMirror),
                '\\' => Ok(Point::BackwardMirror),
                '|' => Ok(Point::VerticalSplit),
                '-' => Ok(Point::HorizontalSplit),
                _ => Err(ParseError::InvalidFormat("valid tile character")),
            })
            .collect::<ParseResult<Vec<Point>>>()?;

        grid.push_row(row);
    }

    Ok(grid)
}

/// Part 1
/// ------
///
/// After firing a beam in the northwestern
/// corner, how many cells are illuminated?
fn part_one(data: &Grid<Point>) -> usize {
    let start = Beam { direction: Direction::East, position: GridIndex::new(0, 0) };
    light_cells(start, data)
}

fn light_cells(start: Beam, data: &Grid<Point>) -> usize {
    let mut beams = VecDeque::from(vec![start]);

    let mut is_lit: Grid<bool> = Grid::new_default(data.width(), data.height());
    let mut done: HashSet<Beam> = HashSet::new();

    while let Some(current) = beams.pop_front() {
        // println!(
        //     "Current state: {:?} at {:?} with {} beams",
        //     current,
        //     data[current.position],
        //     beams.len()
        // );

        if done.contains(&current) {
            continue;
        }

        // let current = beam.clone();

        // Adjust this cell.
        is_lit[current.position] = true;
        done.insert(current.clone());

        // Check the current tile.
        match data[current.position] {
            Point::Empty => {
                if let Some(next) = current.advance(data) {
                    beams.push_back(next);
                }
            }
            Point::ForwardMirror => {
                if let Some(next) = current.rotate_forward().advance(data) {
                    beams.push_back(next);
                }
            }
            Point::BackwardMirror => {
                if let Some(next) = current.rotate_back().advance(data) {
                    beams.push_back(next);
                }
            }
            Point::VerticalSplit => match current.direction {
                Direction::North | Direction::South => {
                    if let Some(next) = current.advance(data) {
                        beams.push_back(next);
                    }
                }
                Direction::East | Direction::West => {
                    if let Some(one) = data.up_index(current.position) {
                        beams.push_back(Beam {
                            direction: Direction::North,
                            position: one,
                        });
                    }
                    if let Some(position) = data.down_index(current.position) {
                        beams.push_back(Beam {
                            direction: Direction::South,
                            position,
                        });
                    }
                }
            },
            Point::HorizontalSplit => match current.direction {
                Direction::North | Direction::South => {
                    if let Some(position) = data.left_index(current.position) {
                        beams.push_back(Beam {
                            direction: Direction::West,
                            position,
                        });
                    }
                    if let Some(position) = data.right_index(current.position) {
                        beams.push_back(Beam {
                            direction: Direction::East,
                            position,
                        });
                    }
                }
                Direction::East | Direction::West => {
                    if let Some(next) = current.advance(data) {
                        beams.push_back(next)
                    }
                    // do nothing
                }
            },
        }
    }

    is_lit.cell_iter().filter(|&point| *point).count()
}

#[allow(unused)]
fn part_two() {
    unimplemented!();
}

fn main() {
    let input = read_to_string("src/input/day16.txt").expect("Could not read input");
    let data = parse_input(&input).expect("Parsing failed");

    println!("The number of illuminated cells is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day16-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        let expected = [
            vec![Point::Empty],
            vec![Point::VerticalSplit],
            vec![Point::Empty; 3],
            vec![Point::BackwardMirror],
            vec![Point::Empty; 4],
        ]
        .concat();
        let actual = data.row_iter(0).map(|&point| point).collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day16-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        println!("Width: {}, Height: {}", data.width(), data.height());
        assert_eq!(part_one(&data), 46);
    }
}
