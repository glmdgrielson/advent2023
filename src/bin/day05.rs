//! Day 05 of Advent of Code 2023
//!
//! If You Give A Seed A Fertilizer
//! ===============================
//!
//! We need to do some gardening today.
//!
//! Oh boy, @#$% me! We've got complicated input today!

use std::fs::read_to_string;
use std::ops::Range;

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
struct Puzzle {
    pub seeds: Vec<u32>,
    pub mappings: Vec<Vec<Mapping>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Mapping {
    /// The start of the destination range.
    pub dest: u32,
    /// The start of the source range, which comes SECOND.
    pub src: u32,
    /// The length of both ranges.
    pub len: u32,
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
        .collect::<Result<Vec<u32>, ParseError>>()?;

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
fn part_one(data: &Puzzle) -> Option<u32> {
    data.seeds.iter().map(|seed| map_seed(&data, *seed)).min()
}

fn map_seed(data: &Puzzle, seed: u32) -> u32 {
    data.mappings.iter().fold(seed, |loc, map| {
        let mapping = map.iter().find_map(|map| map_step(loc, map));
        // println!("Current value is: {:?}", mapping);
        match mapping {
            Some(loc) => loc,
            None => loc,
        }
    })
}

fn map_step(loc: u32, map: &Mapping) -> Option<u32> {
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
fn part_two(data: &Puzzle) -> u32 {
    let ranges = data
        .seeds
        .chunks_exact(2)
        .map(|range| {
            let start = range[0];
            let length = range[1];

            start..start + length
        })
        .collect::<Vec<_>>();

    unimplemented!()
}

fn map_range(maps: &[Mapping], range: &Range<u32>) -> Option<Vec<Range<u32>>> {
    // Collect all of the mappings relevant to this range.
    let maps: Vec<_> = maps
        .iter()
        // Filter out mappings that start after the range has stopped.
        .filter(|map| range.end >= map.src)
        // Filter out mappings that end after we've started.
        .filter(|map| range.start < map.src + map.len)
        .collect();

    if maps.is_empty() {
        // None of the elements fit this range,
        // so return the ranges we already have.
        None
    } else {
        // We need to add new ranges.
        let mut added_ranges = Vec::new();

        maps.iter().for_each(|map| {
            if range.start < map.src {
                // We start outside of the range,
                // but end inside of it.
                // Add two ranges.

                let top = range.start..map.src;
                let bottom = map.src..range.end;

                let offset = bottom.len();
                let bottom = map.dest..map.dest + offset as u32;

                added_ranges.push(top);
                added_ranges.push(bottom);
            } else if range.end < map.src + map.len {
                // We fall entirely in range.
                // Add one range.

                let start = range.start - map.src;
                let end = range.end - map.src;

                added_ranges.push(map.dest + start..map.dest + end);
            } else if range.start < map.src + map.len {
                // We start in the range, but
                // end up outside of it.
                // Add two ranges.

                let offset = range.start - map.src;
                let length = (map.src + map.len) - range.start;

                let top = map.dest + offset..map.dest + offset + length;
                let bottom = map.src + map.len..range.end;

                added_ranges.push(top);
                added_ranges.push(bottom);
            }
        });

        assert!(!added_ranges.is_empty());

        Some(added_ranges)
    }
}

fn main() {
    let input = read_to_string("src/input/day05.txt").expect("Could not load input");
    let data = parse_input(&input).expect("Parsing failed");

    let one = part_one(&data).expect("Puzzle should have solution");
    println!("The shortest seed location is {}", one);

    let two = part_two(&data);
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
    fn test_step_range() {
        let range = 79..(79 + 14);
        let mappings = vec![
            Mapping {
                dest: 50,
                src: 98,
                len: 2,
            },
            Mapping {
                dest: 52,
                src: 50,
                len: 48,
            },
        ];

        let stepped_ranges = map_range(&mappings, &range);

        assert_eq!(stepped_ranges, Some(vec![81..81 + 14]));
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/input/day05-test.txt").expect("Could not read input");
        let data = parse_input(&input).expect("Parsing failed");

        assert_eq!(part_two(&data), 46);
    }
}
