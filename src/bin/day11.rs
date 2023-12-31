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
/// The data this puzzle uses.
struct Image {
    /// The initial map of galaxies.
    ///
    /// A cell is true if there is a galaxy there,
    /// and false if there is not a galaxy there.
    pub map: Grid<bool>,
    /// A list of all of the galaxies we have.
    ///
    /// This should exactly match the indices
    /// in `map` that have a `true` value.
    pub galaxies: Vec<GridIndex>,
}

impl Image {
    /// Expand this space.
    ///
    /// This uses the Part 1 assumption that
    /// each empty row and column gets expanded
    /// exactly once. This is as such entirely
    /// useless for part 2.
    fn expand(&self) -> Self {
        let width = self.map.width();

        let mut grid = self.map.clone();

        // Find the empty rows and store them in right-to-left order.
        let empty_rows: Vec<usize> = grid
            .rows()
            .filter(|idx| grid.row_iter(*idx).all(|is_galaxy| !is_galaxy))
            .rev()
            .collect();
        for row in empty_rows {
            // Insert the rows. If we hadn't reversed
            // `empty_rows`, this would add rows in the wrong places.
            // Learned that the hard way.
            grid.insert_row(row, vec![false; width]);
        }

        // This is the same algorithm but for columns.
        
        // Make sure we get the CURRENT height after adding rows.
        let height = grid.height();
        let empty_columns: Vec<usize> = grid
            .columns()
            .filter(|idx| grid.column_iter(*idx).all(|is_galaxy| !is_galaxy))
            .rev()
            .collect();
        for column in empty_columns {
            grid.insert_column(column, vec![false; height]);
        }

        // Get the new galaxy positions.
        let galaxies = grid
            .cells_with_indices_iter()
            .filter_map(|(idx, is_galaxy)| if *is_galaxy { Some(idx) } else { None })
            .collect();

        Self {
            map: grid,
            galaxies,
        }
    }

    // Get the indices of rows that expand.
    fn empty_rows(&self) -> Vec<usize> {
        self.map
            .rows()
            .filter(|idx| self.map.row_iter(*idx).all(|is_galaxy| !is_galaxy))
            .collect()
    }

    // Get the indices of columns that expand.
    fn empty_columns(&self) -> Vec<usize> {
        self.map
            .columns()
            .filter(|idx| self.map.column_iter(*idx).all(|is_galaxy| !is_galaxy))
            .collect()
    }
}

#[inline]
/// Find the [Manhattan distance][taxi] of two indices.
///
/// [taxi]: https://en.wikipedia.org/wiki/Taxicab_geometry 
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
                // Map characters to whether they have galaxies.
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

    // Get the list of cells with galaxies.
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

fn part_two(data: &Image) -> usize {
    let image = data;
    let last = image.galaxies.len() - 1;

    let res = image.galaxies[0..=last]
        .iter()
        .enumerate()
        .map(|(idx, one)| {
            (idx..image.galaxies.len())
                .map(|idx| -> usize {
                    let two = image.galaxies[idx];

                    expanded_difference(*one, two, image)
                })
                .sum::<usize>()
        })
        .sum();

    res
}

/// The number of rows or columns to add when expanding for Part 2.
const EXPANSION: usize = 1_000_000 - 1;

/// Expand coordinates and return their distance.
fn expanded_difference(one: GridIndex, two: GridIndex, image: &Image) -> usize {
    let empty_rows = image.empty_rows();
    let empty_columns = image.empty_columns();

    // Check which rows we need to expand.
    let row_diff_one = empty_rows.iter().filter(|&idx| *idx < one.row()).count();
    let row_diff_two = empty_rows.iter().filter(|&idx| *idx < two.row()).count();
    // Do an ad hoc expansion.
    let row_one = one.row() + row_diff_one * EXPANSION;
    let row_two = two.row() + row_diff_two * EXPANSION;

    // Check which columns we need to expand.
    let col_diff_one = empty_columns
        .iter()
        .filter(|&idx| *idx < one.column())
        .count();
    let col_diff_two = empty_columns
        .iter()
        .filter(|&idx| *idx < two.column())
        .count();
    // Again, perform the ad hoc expansion.
    let col_one = one.column() + col_diff_one * EXPANSION;
    let col_two = two.column() + col_diff_two * EXPANSION;

    // Get the distance of the now expanded coordinates.
    row_one.abs_diff(row_two) + col_one.abs_diff(col_two)
}

fn main() {
    let input = read_to_string("src/input/day11.txt").expect("Could not read input");
    let data = parse_input(&input).expect("Parsing failed");

    println!(
        "The sum distance of all of the galaxies is {}",
        part_one(&data)
    );
    println!(
        "The sum distance of all galaxies in massive space is {}",
        part_two(&data)
    );
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

    // No test for part two because we aren't actually given that information.

    #[test]
    #[ignore = "Relying on external source for this one, number may be wrong"]
    fn test_part_two() {
        let input = read_to_string("src/input/day11-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing should succeed");

        assert_eq!(part_two(&data), 82_000_210);
    }
}
