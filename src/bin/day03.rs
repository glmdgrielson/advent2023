//! Day 03 of Advent of Code 2023
//!
//! Gear Ratios
//! ===========
//! The gondola is out and you need
//! to figure out all of the parts!

use std::collections::HashMap;
use std::fs::read_to_string;

use simple_grid::{Grid, GridIndex};

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
            for idx in idx.neighbors() {
                if grid.contains_index(idx) {
                    let sym = grid[idx];
                    if is_symbol(sym) {
                        num.symbol = Some(idx);
                    }
                }
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

fn is_symbol(ch: char) -> bool {
    match ch {
        // numbers are not symbol characters
        '0'..='9' => false,
        // Periods are empty space, not symbol characters.
        '.' => false,
        _ => true,
    }
}

fn part_one(grid: &Grid<char>) -> u32 {
    let data = parse_grid(&grid);

    data.iter()
        .filter(|n| n.symbol.is_some())
        .fold(0, |acc, n| acc + n.number)
}

fn part_two(grid: &Grid<char>) -> u32 {
    let mut sum = 0;

    let numbers = parse_grid(grid);
    // Filter the grid numbers to just the ones adjacent to stars.
    let stars = numbers
        .iter()
        .filter(|n| n.symbol.is_some_and(|idx| grid[idx] == '*'))
        .map(|n| n.symbol.unwrap());

    // Create a mapping between the index and the numbers
    // beside it.
    let mut map: HashMap<_, _> = HashMap::new();
    for star in stars {
        let value = numbers
            .iter()
            // Check for all of the numbers for which this
            // index is the part of.
            .filter(|n| n.symbol.is_some_and(|idx| idx == star))
            // Turn this into a reasonable value.
            .collect::<Vec<_>>();

        map.insert(star, value);
    }

    for value in map.values() {
        // Check that this is adjacent to exactly two numbers.
        if value.len() == 2 {
            // Get the gear ratio and add it to the sum.
            sum += value[0].number * value[1].number;
        }
    }

    sum
}

fn main() {
    let input = read_to_string("src/input/day03.txt").expect("Could not read input");
    let grid = parse_input(&input);

    println!("The total sum of the part numbers is {}", part_one(&grid));
    println!("The sum of all of the gear ratios is {}", part_two(&grid));
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

        let two = data.get(1).unwrap();
        assert_eq!(two.number, 114);
        assert!(two.symbol.is_none());
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day03-test.txt").expect("Could not read example");
        let grid = parse_input(&input);

        assert_eq!(part_one(&grid), 4361);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day03-test.txt").expect("Could not read example");
        let grid = parse_input(&input);

        assert_eq!(part_two(&grid), 467835);
    }
}
