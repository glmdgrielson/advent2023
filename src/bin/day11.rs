//! Day 11 of Advent of Code
//!
//! Cosmic Expansion
//! ================
//!
//! This elf wants us to help
//! out with understanding the
//! stars. Weird how the cosmic
//! expansion seems to be bugged.

use std::fs::read_to_string;

use simple_grid::{Grid, GridIndex};

use advent_2023::ParseError;

#[derive(Clone, Debug, PartialEq)]
struct Image {
    pub map: Grid<bool>,
    pub galaxies: Vec<GridIndex>,
}

impl Image {
    fn expand(&self) -> Self {
        let width = self.map.width();

        let mut grid = self.map.clone();

        let empty_rows: Vec<usize> = grid
            .rows()
            .filter(|idx| grid.row_iter(*idx).all(|is_galaxy| !is_galaxy))
            .rev()
            .collect();
        for row in empty_rows {
            grid.insert_row(row, vec![false; width]);
        }

        let height = grid.height();
        let empty_columns: Vec<usize> = grid
            .columns()
            .filter(|idx| grid.column_iter(*idx).all(|is_galaxy| !is_galaxy))
            .rev()
            .collect();
        for column in empty_columns {
            grid.insert_column(column, vec![false; height]);
        }

        let galaxies = grid
            .cells_with_indices_iter()
            .filter_map(|(idx, is_galaxy)| if *is_galaxy { Some(idx) } else { None })
            .collect();

        Self {
            map: grid,
            galaxies,
        }
    }
}

#[inline]
fn index_distance(one: GridIndex, two: GridIndex) -> usize {
    one.row().abs_diff(two.row()) + one.column().abs_diff(two.column())
}

/// Input consists of a grid of characters, either a `.`
/// signifying empty space or `#` signifying a galaxy.
fn parse_input(input: &str) -> Result<Image, ParseError> {
    let mut grid = Grid::new(0, 0, vec![]);

    let data = input
        .lines()
        .map(|line| {
            let res = line
                .chars()
                .map(|ch| match ch {
                    '#' => Ok(true),
                    '.' => Ok(false),
                    _ => Err(ParseError::InvalidFormat("valid character")),
                })
                .collect::<Result<Vec<bool>, ParseError>>()?;
            Ok(res)
        })
        .collect::<Result<Vec<Vec<bool>>, ParseError>>()?;

    for row in data {
        grid.push_row(row);
    }

    let stars = grid
        .cells_with_indices_iter()
        .filter_map(|(idx, is_galaxy)| if *is_galaxy { Some(idx) } else { None })
        .collect::<Vec<_>>();

    Ok(Image {
        map: grid,
        galaxies: stars,
    })
}

/// Part 1
/// ------
///
/// After the galaxy has expanded, find the distances
/// from one galaxy to another and sum them up.
fn part_one(data: &Image) -> usize {
    let image = data.expand();
    let last = image.galaxies.len() - 1;

    let res = image.galaxies[0..=last]
        .iter()
        .enumerate()
        .map(|(idx, galaxy)| {
            (idx..image.galaxies.len())
                .map(|idx| index_distance(*galaxy, image.galaxies[idx]))
                .sum::<usize>()
        })
        .sum();

    res
}

#[allow(unused)]
fn part_two(data: &Image) {
    todo!();
}

fn main() {
    let input = read_to_string("src/input/day11.txt").expect("Could not read input");
    let data = parse_input(&input).expect("Parsing failed");

    println!("The sum distance of all of the galaxies is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day11-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        for galaxy in data.galaxies {
            assert!(data.map[galaxy], "Galaxy data at {} corrupted", galaxy);
        }
    }

    #[test]
    fn test_image_expand() {
        let input = read_to_string("src/input/day11-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        let image = data.expand();
        for row in [3, 4, 8, 9] {
            assert!(
                image.map.row_iter(row).all(|is_galaxy| !is_galaxy),
                "Row {} is not empty",
                row
            );
        }
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day11-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert_eq!(part_one(&data), 374);
    }
}
