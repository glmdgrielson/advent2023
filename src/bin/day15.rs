//! Day 15 of Advent of Code
//!
//! Lens Library
//! ============
//!
//! This one looks simple enough,
//! it's just a simple hashing
//! algorithm to deal with...

use std::collections::HashMap;
use std::fs::read_to_string;

use advent_2023::{ParseError, ParseResult};

/// Input consists of a comma separated
/// list of strings to hash together.
///
/// For once, this function is infalliable!
fn parse_input<'i>(input: &'i str) -> Vec<&'i str> {
    input.trim().split(',').collect()
}

const FACTOR: u32 = 17;

const REDUCTION: u32 = 256;

fn hashvent(input: &str) -> u32 {
    input.chars().fold(0, |acc, ch| hashvent_step(ch, acc))
}

fn hashvent_step(ch: char, acc: u32) -> u32 {
    let hash: u32 = ch.into();
    let val = acc + hash;
    let val = val * FACTOR;
    val % REDUCTION
}

/// Part 1
/// ------
///
/// What is the sum of all of the hashes in the results?
fn part_one(data: &[&str]) -> u32 {
    data.iter().map(|&i| hashvent(i)).sum()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Instruction<'a> {
    pub label: &'a str,
    pub operation: Operation,
}

impl<'i> TryFrom<&'i str> for Instruction<'i> {
    type Error = ParseError;

    fn try_from(value: &'i str) -> Result<Self, Self::Error> {
        match value.find(&['-', '=']) {
            None => Err(ParseError::InvalidFormat("valid operation character")),
            Some(position) => {
                let (label, operation) = value.split_at(position);
                let operation = if let Some(focus) = operation.strip_prefix('=') {
                    let focus: usize = focus.parse()?;
                    if !(1..=9).contains(&focus) {
                        return Err(ParseError::InvalidFormat("valid focal length"));
                    }
                    Operation::Insert(focus)
                } else {
                    // We assume that if the first pass didn't succeed,
                    // this is the OTHER operation
                    Operation::Remove
                };
                // let operation = match value[position] {
                //     '-' => {}
                //     '=' => {}
                //     _ => unreachable!()
                // };
                Ok(Instruction { label, operation })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Operation {
    /// Insert a new lens
    Insert(usize),
    /// Remove a lens
    Remove,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Lens<'a>(&'a str, usize);

/// Part 2
/// ------
///
/// So it turns out the codes we
/// were hashing were meaningful. Whoops.
/// What is the result of applying the
/// HASHMAP algorithm using the codes
/// we were provided?
fn part_two(data: &[&str]) -> ParseResult<usize> {
    let steps = data
        .iter()
        .map(|&step| step.try_into())
        .collect::<ParseResult<Vec<Instruction>>>()?;

    let map = mapvent(steps);
    Ok(map
        .into_iter()
        .map(|(slot, lenses)| -> usize {
            lenses
                .iter()
                .enumerate()
                .map(|(idx, lens)| (slot + 1) * (idx + 1) * lens.1)
                .sum()
        })
        .sum())
}

fn mapvent<'a>(steps: Vec<Instruction<'a>>) -> HashMap<usize, Vec<Lens<'a>>> {
    let mut map = HashMap::new();
    for step in steps {
        let slot = hashvent(step.label) as usize;
        match step.operation {
            Operation::Insert(focus) => {
                map.entry(slot)
                    .and_modify(|lenses: &mut Vec<Lens<'a>>| {
                        let position = lenses.iter().position(|lens| lens.0 == step.label);
                        match position {
                            Some(position) => {
                                // This label already exists.
                                // Replace the focus.
                                lenses[position].1 = focus;
                            }
                            None => {
                                // Add this label and focus.
                                lenses.push(Lens(step.label, focus));
                            }
                        }
                    })
                    .or_insert(vec![Lens(step.label, focus)]);
            }
            Operation::Remove => {
                map.entry(slot).and_modify(|lenses| {
                    let position = lenses.iter().position(|lens| lens.0 == step.label);
                    if let Some(position) = position {
                        lenses.remove(position);
                    }
                });
            }
        }
    }
    map
}

fn main() {
    let input = read_to_string("src/input/day15.txt").expect("Could not read input");
    let data = parse_input(&input);

    println!("The total sum of the hashes is {}", part_one(&data));
    println!("The total focusing power is {}", part_two(&data).expect("Mapping should succeed"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashvent() {
        assert_eq!(hashvent("HASH"), 52);
    }

    #[test]
    fn test_hashvent_step() {
        assert_eq!(hashvent_step('H', 0), 200);
        assert_eq!(hashvent_step('A', 200), 153);
        assert_eq!(hashvent_step('S', 153), 172);
        assert_eq!(hashvent_step('H', 172), 52);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day15-test.txt").expect("Could not read example");
        let data = parse_input(&input);

        assert_eq!(part_one(&data), 1320);
    }

    #[test]
    fn test_tryfrom_step() {
        let input = read_to_string("src/input/day15-test.txt").expect("Could not read example");
        let data = parse_input(&input);

        let steps = data
            .iter()
            .map(|&step| step.try_into())
            .collect::<Result<Vec<Instruction>, _>>();
        let steps = steps.unwrap();

        assert_eq!(
            steps[0],
            Instruction {
                label: "rn",
                operation: Operation::Insert(1)
            }
        );
        assert_eq!(
            steps[1],
            Instruction {
                label: "cm",
                operation: Operation::Remove
            }
        );
    }

    #[test]
    fn test_mapvent() {
        let input = read_to_string("src/input/day15-test.txt").expect("Could not read example");
        let data = parse_input(&input);

        let steps = data
            .iter()
            .map(|&step| step.try_into())
            .collect::<Result<Vec<Instruction>, _>>()
            .expect("Steps should convert");
        let map = mapvent(steps);

        assert_eq!(map.get(&0), Some(&vec![Lens("rn", 1), Lens("cm", 2)]));
        assert_eq!(
            map.get(&3),
            Some(&vec![Lens("ot", 7), Lens("ab", 5), Lens("pc", 6)])
        );
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day15-test.txt").expect("Could not read example");
        let data = parse_input(&input);

        assert_eq!(part_two(&data), Ok(145));
    }
}
