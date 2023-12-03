//! Day 03 of Advent of Code 2023
//!
//! Gear Ratios
//! ===========
//! The gondola is out and you need
//! to figure out all of the parts!

use simple_grid::Grid;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct GridNumber {
    /// The value of this GridNumber.
    pub number: u32,
    /// The symbol denoting that this is a
    /// part if Some, otherwise None.
    pub symbol: Option<char>,
}

/// Input consists of a grid consisting
/// of numbers, symbols, and periods.
fn parse_input(input: &str) -> Vec<GridNumber> {
    let mut res = Vec::new();

    // Create an empty grid
    let mut grid = Grid::<char>::new(0, 0, Vec::new());
    // Make this grid reflect the input.
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

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
                        num.symbol = Some(sym);
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

fn part_one(data: &[GridNumber]) -> u32 {
    data.iter()
        .filter(|n| n.symbol.is_some())
        .fold(0, |acc, n| acc + n.number)
}

#[allow(dead_code)]
fn part_two() {
    todo!("Part one incomplete!")
}

fn main() {
    let input = read_to_string("src/input/day03.txt").expect("Could not read input");
    let data = parse_input(&input);

    println!("The total sum of the part numbers is {}", part_one(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day03-test.txt").expect("Could not read example");
        let data = parse_input(&input);

        // Check that there _is_ a grid number.
        let one = data.get(0).unwrap();
        // Check the value of the grid number.
        assert_eq!(one.number, 467);
        // Check that it's a part adorned with '*'.
        assert_eq!(one.symbol, Some('*'));

        let two = data.get(1).unwrap();
        assert_eq!(two.number, 114);
        assert_eq!(two.symbol, None);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day03-test.txt").expect("Could not read example");
        let data = parse_input(&input);

        assert_eq!(part_one(&data), 4361);
    }
}
