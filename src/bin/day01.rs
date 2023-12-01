/// Day one of Advent of Code
use std::fs::read_to_string;

/// Find the first and last digits of each line and add them.
fn part_one(data: String) -> usize {
    let mut sum = 0;
    for line in data.lines() {
        // Find the first digit.
        let one = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found");
        // Find the last digit.
        let two = line
            .chars()
            .rfind(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found");

        // Since `one` is the tens digit, multiply it and add
        // `two to the sum.
        let total = one * 10 + two;
        sum += total as usize;
        // let one
    }
    return sum;
}

fn main() {
    let input = read_to_string("input/day01.txt").expect("File could not be read.");

    println!("Part one solution is: {}", part_one(input));
    // let mut lines = input.lines();
    // print!("{}", lines.next().expect("Missing data"));
    //
}
