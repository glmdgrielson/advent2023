//! Day 05 of Advent of Code 2023
//!
//! If You Give A Seed A Fertilizer
//! ===============================
//!
//! We need to do some gardening today.
//!
//! Oh boy, @#$% me! We've got complicated input today!

// Putting this in here because @#$% this puzzle.
#![allow(unused)]

use std::collections::VecDeque;
use std::fs::read_to_string;
use std::ops::Range;

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
struct Puzzle {
    pub seeds: Vec<u64>,
    pub mappings: Vec<Vec<Mapping>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Mapping {
    /// The start of the destination range.
    pub dest: u64,
    /// The start of the source range, which comes SECOND.
    pub src: u64,
    /// The length of both ranges.
    pub len: u64,
}

#[derive(Error, Clone, Debug, PartialEq)]
enum ParseError<'a> {
    #[error("Expected to see '{0}', found {1}")]
    InvalidFormat(&'static str, &'a str),
    #[error("Failed to parse number: {0}")]
    ParseFailed(#[from] ParseIntError),
}

/// Input consists of a set of seeds, and a list of mappings.
fn parse_input(input: &str) -> Result<Puzzle, ParseError> {
    // Split the file by blank lines.
    let Some((seeds, sections)) = input.split_once("\n\n") else {
        return Err(ParseError::InvalidFormat("`seeds:` section", &input));
    };

    let Some(seeds) = seeds.strip_prefix("seeds: ") else {
        return Err(ParseError::InvalidFormat("seeds: ", seeds));
    };
    let seeds = seeds
        .split_whitespace()
        .map(|seed| seed.parse().map_err(ParseError::ParseFailed))
        .collect::<Result<Vec<u64>, ParseError>>()?;

    let mappings = sections
        .trim()
        .split("\n\n")
        .map(|section| {
            let Some((_name, maps)) = section.split_once("\n") else {
                return Err(ParseError::InvalidFormat("map title", section));
            };

            // let Some(name) = name.strip_suffix(" map:") else {
            //     return Err(ParseError::InvalidFormat("map:", name));
            // };

            let maps: Vec<Mapping> = maps
                .lines()
                .map(|line| {
                    let range: Vec<_> = line.split_whitespace().collect();

                    if range.len() != 3 {
                        return Err(ParseError::InvalidFormat("proper range format", line));
                    }

                    let dest = range[0].parse().map_err(ParseError::ParseFailed)?;
                    let src = range[1].parse().map_err(ParseError::ParseFailed)?;
                    let len = range[2].parse().map_err(ParseError::ParseFailed)?;

                    Ok(Mapping { dest, src, len })
                })
                .collect::<Result<Vec<Mapping>, ParseError>>()?;

            Ok(maps)
        })
        .collect::<Result<Vec<Vec<Mapping>>, ParseError>>()?;

    Ok(Puzzle { seeds, mappings })
}

/// Part 1
/// ------
///
/// Which seed can we plant first?
fn part_one(data: &Puzzle) -> Option<u64> {
    data.seeds.iter().map(|seed| map_seed(&data, *seed)).min()
}

fn map_seed(data: &Puzzle, seed: u64) -> u64 {
    data.mappings.iter().fold(seed, |loc, map| {
        let mapping = map.iter().find_map(|map| map_step(loc, map));
        // println!("Current value is: {:?}", mapping);
        match mapping {
            Some(loc) => loc,
            None => loc,
        }
    })
}

fn map_step(loc: u64, map: &Mapping) -> Option<u64> {
    if loc >= map.src {
        let offset = loc - map.src;
        if offset < map.len {
            Some(map.dest + offset)
        } else {
            None
        }
    } else {
        None
    }
}

/// Part 2
/// ------
///
/// It turns out we weren't handed a list of
/// individual seeds. Oh no, we were handed
/// a list of _ranges_ of seeds. Of these ranges,
/// what's the location of the seed we're planting first?
fn part_two(data: &Puzzle) -> Option<u64> {
    let ranges = data
        .seeds
        .chunks_exact(2)
        .map(|range| {
            let start = range[0];
            let length = range[1];

            start..start + length
        })
        .collect::<Vec<_>>();

    let mut curr_ranges = ranges.clone();

    for maps in data.mappings.iter() {
        let mut new_ranges = vec![];

        for range in curr_ranges.iter() {
            let mut curr = range.clone();

            for rule in maps {
                let applies = curr.start <= curr.end
                    && curr.start <= rule.src + rule.len
                    && curr.end >= rule.src;

                if applies {
                    if curr.start < rule.src {
                        // We start before the range.
                        new_ranges.push(curr.start..rule.src);
                        curr.start = rule.src;
                        if curr.end < rule.src + rule.len {
                            // We end within the range.
                            let offset = curr.end - curr.start;
                            new_ranges.push(rule.dest..rule.dest + offset);
                            // This is a dud operation,
                            // meant to ensure this range gets discarded.
                            curr.start = curr.end + 1;
                        } else {
                            // We end outside of the range, so we stop here.
                            new_ranges.push(rule.dest..rule.dest + rule.len);
                            curr.start = rule.src + rule.len;
                        }
                    } else if curr.end < rule.src + rule.len {
                        // We start and end inside of the range.
                        let start = curr.start - rule.src;
                        let end = curr.end - rule.src;
                        new_ranges.push(rule.dest + start..rule.dest + end);
                        // This is a dud operation, meant to clear this range.
                        curr.start = curr.end + 1;
                    } else {
                        // We start within the range and end outside of it.
                        let offset = curr.start - rule.src;
                        new_ranges.push(rule.dest + offset..rule.dest + rule.len);
                        curr.start = rule.src + rule.len;
                    }
                }
            }

            // If there's still a range left, add it to the list.
            if curr.start < curr.end {
                new_ranges.push(curr.clone());
            }
        }

        // new_ranges.sort_by_key(|range| range.start);
        curr_ranges = new_ranges;
    }

    println!("{:?}", curr_ranges);
    let starts = curr_ranges
        .iter()
        .map(|range| range.start)
        .collect::<Vec<_>>();
    if cfg!(not(test)) {
        assert!(starts.contains(&60294664));
    }
    let min = curr_ranges.into_iter().min_by_key(|range| range.start);

    min.map(|range| range.start)
}

fn split_range(range: &Range<u64>, map: &Mapping) -> Vec<Range<u64>> {
    vec![]
}

fn main() {
    let input = read_to_string("src/input/day05.txt").expect("Could not load input");
    let data = parse_input(&input).expect("Parsing failed");

    let one = part_one(&data).expect("Puzzle should have solution");
    println!("The shortest seed location is {}", one);

    let two = part_two(&data).expect("Ranges should exist");
    assert_eq!(two, 60294664);
    println!("The earliest seed location with ranges is {}", two);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_to_string("src/input/day05-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(data.seeds, vec![79, 14, 55, 13]);

        let soil = &data.mappings[0];
        assert_eq!(
            soil,
            &vec![
                Mapping {
                    dest: 50,
                    src: 98,
                    len: 2
                },
                Mapping {
                    dest: 52,
                    src: 50,
                    len: 48
                },
            ]
        );
    }

    #[test]
    fn test_map_step() {
        let input = read_to_string("src/input/day05-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing failed");

        let mapping = &data.mappings[0];
        let loc = 79;
        let mapping = mapping
            .iter()
            .find(|map| loc >= map.src && loc < map.src + map.len)
            .unwrap();

        assert_eq!(map_step(loc, mapping), Some(81));
    }

    #[test]
    fn test_map_seed() {
        let input = read_to_string("src/input/day05-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(map_seed(&data, 79), 82);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("src/input/day05-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing failed");

        let res = part_one(&data);

        assert_eq!(res, Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day05-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_two(&data), Some(46));
    }
}
