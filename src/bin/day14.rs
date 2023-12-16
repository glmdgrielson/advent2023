//! Day 14 of Advent of Code
//!
//! Parabolic Reflector Dish
//! ========================
//!
//! There's a, well, parabolic
//! reflector dish that's a bit
//! out of shape. We need to fix
//! it before things can get
//! back to normal.

use std::fs::read_to_string;

use simple_grid::Grid;

use advent_2023::{ParseError, ParseResult};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rock {
    Ground,
    Round,
    Cube,
}

/// Input consists of a grid of
/// rocks and ground. Ground is
/// signified by a `.` character,
/// while rocks can be either `#`
/// or `O` depending on whether it
/// is cubed or rounded.
fn parse_input(input: &str) -> ParseResult<Grid<Rock>> {
    let mut grid = Grid::new(0, 0, Vec::new());

    for line in input.lines() {
        let rocks = line
            .chars()
            .map(|ch| match ch {
                '.' => Ok(Rock::Ground),
                '#' => Ok(Rock::Cube),
                'O' => Ok(Rock::Round),
                _ => Err(ParseError::InvalidFormat("proper rock character")),
            })
            .collect::<ParseResult<Vec<Rock>>>()?;

        grid.push_row(rocks);
    }

    Ok(grid)
}

fn tilt_north(grid: &Grid<Rock>) -> Grid<Rock> {
    let mut out = Grid::new(0, 0, vec![]);

    for col in grid.columns() {
        let column = grid.column_iter(col).map(|&rock| rock).collect::<Vec<_>>();

        let chunks = column
            .split(|&rock| rock == Rock::Cube)
            .map(|chunk| {
                let (rounds, grounds): (Vec<Rock>, Vec<Rock>) =
                    chunk.iter().partition(|&rock| *rock == Rock::Round);
                [rounds, grounds].concat()
            })
            .collect::<Vec<Vec<_>>>();

        let column = chunks.join(&Rock::Cube);

        out.push_column(column);
    }

    assert_eq!(grid.dimensions(), out.dimensions());
    out
}

/// Part 1
/// ------
///
/// Tilt the plane so that all of the
/// rocks move upward. How much stress
/// does this action place upon the dish?
fn part_one(data: &Grid<Rock>) -> usize {
    let grid = tilt_north(data);

    grid.rows()
        .map(|row| {
            let count = grid
                .row_iter(row)
                .filter(|&rock| *rock == Rock::Round)
                .count();
            count * (grid.height() - row)
        })
        .sum()
}

const REPETITIONS: u32 = 1_000_000_000;

/// Detect cycles for a function
fn detect_cycle<T, F>(f: F, initial: T) -> (u32, u32)
where
    F: Fn(T) -> T,
    T: PartialEq<T> + Clone,
{
    let mut power = 1;
    let mut cycle_length = 1;
    let mut slow = initial.clone();
    let mut fast = f(initial.clone());

    while fast != slow {
        if cycle_length == power {
            slow = fast.clone();
            power *= 2;
            cycle_length = 0;
        }
        fast = f(fast);
        cycle_length += 1;
    }

    slow = initial.clone();
    fast = initial.clone();

    for _ in 0..cycle_length {
        fast = f(fast);
    }

    let mut cycle_start = 0;
    while fast != slow {
        slow = f(slow);
        fast = f(fast);
        cycle_start += 1;
    }

    (cycle_length, cycle_start)
}

fn tilt_cycle(grid: Grid<Rock>) -> Grid<Rock> {
    let mut out = grid.clone();
    for _ in 0..4 {
        out = tilt_north(&out);
        out.rotate_cw();
    }
    out
}

/// Part 2
fn part_two(data: &Grid<Rock>) -> usize {
    let (length, start) = detect_cycle(tilt_cycle, data.clone());
    let length = start + (REPETITIONS - start) % length;
    let grid = (0..length).fold(data.clone(), |acc, _| tilt_cycle(acc));

    grid.rows()
        .map(|row| {
            let count = grid
                .row_iter(row)
                .filter(|&rock| *rock == Rock::Round)
                .count();
            count * (grid.height() - row)
        })
        .sum()
}

fn main() {
    let input = read_to_string("src/input/day14.txt").expect("Could not read input");
    let data = parse_input(&input).expect("Parsing failed");

    println!("The total load on the supports is {}", part_one(&data));
    println!(
        "The total load after a billion cycles is {}",
        part_two(&data)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day14-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        let row = data.row_iter(0).collect::<Vec<_>>();
        assert_eq!(row[0], &Rock::Round);
        assert_eq!(row[5], &Rock::Cube);
    }

    #[test]
    fn test_tilt_grid() {
        let input = read_to_string("src/input/day14-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        let grid = tilt_north(&data);

        let actual = grid.column_iter(0).map(|&rock| rock).collect::<Vec<_>>();
        let expected: Vec<_> = [
            vec![Rock::Round; 4],
            vec![Rock::Ground; 4],
            vec![Rock::Cube; 2],
        ]
        .concat();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day14-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_one(&data), 136);
    }

    #[test]
    fn test_detect_cycle() {
        fn test_func(val: u32) -> u32 {
            (val * val + 1) % 255
        }

        assert_eq!(detect_cycle(test_func, 3), (6, 2));
    }

    #[test]
    fn test_tilt_cycle() {
        let input = read_to_string("src/input/day14-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        let grid = tilt_cycle(data);

        println!("{:?}", grid.row_iter(0).collect::<Vec<_>>());
        assert_eq!(grid.row_iter(0).position(|&rock| rock == Rock::Round), None);
        assert_eq!(
            grid.row_iter(1).position(|&rock| rock == Rock::Round),
            Some(8)
        );
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day14-test.txt").expect("Could not read example");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_two(&data), 64);
    }
}
