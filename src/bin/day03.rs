//! Day 03 of Advent of Code 2023
//!
//! Gear Ratios
//! ===========
//! The gondola is out and you need
//! to figure out all of the parts!
#![warn(missing_docs)]

use std::collections::HashMap;
use std::fs::read_to_string;

use simple_grid::{Grid, GridIndex};

/// A number found within the grid given as
/// input.
#[derive(Clone, Debug, PartialEq, Default)]
struct GridNumber {
    /// The value of this GridNumber.
    pub number: u32,
    /// The index of the symbol denoting that this is a
    /// part if Some, otherwise None.
    pub symbol: Option<GridIndex>,
}

/// Input consists of a grid consisting
/// of numbers, symbols, and periods.
fn parse_input(input: &str) -> Grid<char> {
    // Create an empty grid
    let mut grid = Grid::<char>::new(0, 0, Vec::new());
    // Make this grid reflect the input.
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    parse_grid(&grid);

    grid
}

/// The puzzle doesn't care about the grid, per se,
/// as so much as it cares about numbers found within
/// the grid. As such, we need to extract the numbers
/// from within the grid.
///
/// We care about two things with each number:
/// the actual _value_ of the number and whether there
/// is a "symbol" somewhere next to it. As such, this
/// function returns a list of structs with both of these
/// features intact.
fn parse_grid(grid: &Grid<char>) -> Vec<GridNumber> {
    let mut res = Vec::new();
    // Create an empty GridNumber.
    let mut num = GridNumber {
        number: 0,
        symbol: None,
    };
    // Iterate over the grid.
    for (idx, ch) in grid.cells_with_indices_iter() {
        if let Some(digit) = ch.to_digit(10) {
            // This is a digit of some sort.

            // Append this digit to the current number.
            // Since we're storing this as an integer,
            // that means multiplying the current value
            // by ten and adding the digit we just found.
            num.number = num.number * 10 + digit;

            // Check for symbols.
            let sym = idx
                .neighbors()
                .find(|&idx| grid.contains_index(idx) && is_symbol(grid[idx]));
            // If we have a symbol and there isn't already a symbol
            // here, add it to this part. ...the puzzle must assume
            // that each part number can only be adjacent to one
            // symbol, somehow. Interesting.
            if sym.is_some_and(|_| num.symbol.is_none()) {
                num.symbol = sym;
            }
        } else {
            // We are looking at something else and we need
            // to cap off the current GridNumber.
            if num.number != 0 {
                // Add this GridNumber to the list.
                res.push(num);
                // Reset num.
                num = GridNumber {
                    number: 0,
                    symbol: None,
                };
            }
        }
    }

    res
}

/// Check to see if this character is a "symbol" as
/// defined by the standards of Part 1 of the puzzle.
fn is_symbol(ch: char) -> bool {
    match ch {
        // numbers are not symbol characters
        '0'..='9' => false,
        // Periods are empty space, not symbol characters.
        '.' => false,
        _ => true,
    }
}

/// Part 1
/// ------
///
/// We need to find the sum of all of the part numbers.
/// A part number is defined by the number being adjacent
/// to a particular symbol, with "symbol" defined as being
/// any character that is not a period (`.`) or a digit
/// (defined the same way as `char::is_ascii_digit`.
///
/// `parse_grid` takes care of actually finding the numbers
/// with symbols; we just need to check whether the symbol
/// is actually _there_.
fn part_one(data: &[GridNumber]) -> u32 {
    data.iter()
        // Reduce the list to the subset of
        // GridNumber that refer to part numbers,
        // which is defined here by having a
        // `Some` value for its symbol.
        .filter(|n| n.symbol.is_some())
        // Sum up all of the values of the
        // found part numbers and return it.
        .fold(0, |acc, n| acc + n.number)
}

/// Part 2
/// ------
///
/// Now we care about gears, which show up on the grid
/// as `'*'` characters. As such, we need to find all of
/// these characters on the grid. But gears have a peculiar
/// property that resulted in a rewrite of the part one code:
/// they care about the numbers adjacent to it. Specifically,
/// an asterisk is only a gear if it is adjacent to exactly
/// **two** numbers on the grid.
///
/// This means that not only do we care about whether a number
/// is a part number, but also whether the symbol is an
/// asterisk and _whether that asterisk is shared with another
/// number on the grid_. This means that `parse_input` has to
/// return the whole grid because we care about the value it
/// returns here.
fn part_two(grid: &Grid<char>, numbers: &[GridNumber]) -> u32 {
    // Get the indices of all the stars in the grid.
    let stars = numbers
        .iter()
        // Filter the grid numbers to just the ones adjacent to stars.
        .filter(|n| n.symbol.is_some_and(|idx| grid[idx] == '*'))
        // Having proven that the symbol exists, we can safely
        // move them out of the option. I could probably do with
        // a `filter_map` here if I understood how that works.
        .map(|n| n.symbol.unwrap());

    // Create a mapping between the index and the numbers
    // beside it.
    let map: HashMap<_, _> = stars
        .map(|star| {
            let value = numbers
                .iter()
                // Find which numbers this star is associated with.
                .filter(|n| n.symbol.is_some_and(|idx| idx == star))
                // Convert into a usable list.
                .collect::<Vec<_>>();
            (star, value)
        })
        .collect();

    map.values().fold(0, |acc, value| {
        // Check if this star is a gear
        if value.len() == 2 {
            // Add the gear ratio to the accumulator.
            acc + value[0].number * value[1].number
        } else {
            // This is _not_ a gear, so we move on.
            acc
        }
    })
}

fn main() {
    let input = read_to_string("src/input/day03.txt").expect("Could not read input");
    let grid = parse_input(&input);
    let data = parse_grid(&grid);

    println!("The total sum of the part numbers is {}", part_one(&data));
    println!(
        "The sum of all of the gear ratios is {}",
        part_two(&grid, &data)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day03-test.txt").expect("Could not read example");
        let grid = parse_input(&input);
        let data = parse_grid(&grid);

        // Check that there _is_ a grid number.
        let one = data.get(0).unwrap();
        // Check the value of the grid number.
        assert_eq!(one.number, 467);
        // Check that it's a part adorned with '*'.
        assert!(one.symbol.is_some());

        // Check that there's more than one grid number.
        let two = data.get(1).unwrap();
        // Check that the second grid number has the value we expect it to.
        assert_eq!(two.number, 114);
        // Check that this grid number is NOT a part.
        assert!(two.symbol.is_none());
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day03-test.txt").expect("Could not read example");
        let grid = parse_input(&input);
        let data = parse_grid(&grid);

        assert_eq!(part_one(&data), 4361);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day03-test.txt").expect("Could not read example");
        let grid = parse_input(&input);
        let data = parse_grid(&grid);

        assert_eq!(part_two(&grid, &data), 467835);
    }
}
