//! Day one of Advent of Code
use std::fs::read_to_string;

/// Find the first and last digits of each line and add them.
fn part_one(data: &str) -> u32 {
    data.lines().fold(0, |acc, line| {
        let one = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found");
        let two = line
            .chars()
            .rfind(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found");

        let total = one * 10 + two;
        acc + total
    })
}

fn part_two(data: &str) -> u32 {
    data.lines().fold(0, |acc, line| {
        let one = find_digit(line);

        let rev: String = line.chars().rev().collect();
        let two = rfind_digit(&rev);

        let total = one * 10 + two;
        acc + total
    })
}

fn find_digit(line: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // Find the number closest to the left.
    let search_l = numbers
        .iter()
        .filter_map(|s| match line.find(s) {
            Some(idx) => Some((s, idx)),
            None => None,
        })
        .min_by_key(|(_, idx)| idx.clone());

    // let one = text_to_number(search_l);
    let one = if let Some((number, index)) = search_l {
        let number = text_to_number(number);

        // We know every line has a digit in it, because part one
        // requires it, so this is safe.
        let digit_idx = line.chars().position(|c| c.is_ascii_digit()).unwrap();
        if digit_idx < index {
            // The digit came first, so we need to turn
            // the index we have into a digit.
            line.chars()
                .nth(digit_idx)
                .map(|c| c.to_digit(10))
                // Verify the mapping worked.
                .unwrap()
                // Verify that we found the digit.
                .unwrap()
        } else {
            number
        }
    } else {
        // This line _has_ no text digit to find
        // so we run the same check from part one.
        line.chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found")
    };
    one
}

/// This is a terrible hack to made the code that WORKS
/// replace the code that doesn't. Essentially, this is
/// the same as `find_digit` except that it works on a
/// reversed copy of the string, so that it can find the
/// LAST instance of a digit.
fn rfind_digit(line: &str) -> u32 {
    let numbers = [
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    // Find the number closest to the left.
    let search = numbers
        .iter()
        .filter_map(|s| match line.find(s) {
            Some(idx) => Some((s, idx)),
            None => None,
        })
        .min_by_key(|(_, idx)| idx.clone());

    // let one = text_to_number(search_l);
    let res = if let Some((number, index)) = search {
        let number = match number {
            &"eno" => 1,
            &"owt" => 2,
            &"eerht" => 3,
            &"ruof" => 4,
            &"evif" => 5,
            &"xis" => 6,
            &"neves" => 7,
            &"thgie" => 8,
            &"enin" => 9,
            _ => unreachable!("Errant backwards number detected"),
        };

        // We know every line has a digit in it, because part one
        // requires it, so this is safe.
        let digit_idx = line.chars().position(|c| c.is_ascii_digit()).unwrap();
        if digit_idx < index {
            // The digit came first, so we need to turn
            // the index we have into a digit.
            line.chars()
                .nth(digit_idx)
                .map(|c| c.to_digit(10))
                // Verify the mapping worked.
                .unwrap()
                // Verify that we found the digit.
                .unwrap()
        } else {
            number
        }
    } else {
        // This line _has_ no text digit to find
        // so we run the same check from part one.
        line.chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found")
    };
    res
}

/// Turn a textual number into a digit.
fn text_to_number(text: &str) -> u32 {
    match text {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!("Errant number found."),
    }
}

fn main() {
    let input = read_to_string("src/input/day01.txt").expect("File could not be read.");

    println!("Part one solution is: {}", part_one(&input));

    println!("Part two solution is: {}", part_two(&input));
    // let mut lines = input.lines();
    // print!("{}", lines.next().expect("Missing data"));
    //
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "This uses the second example input modified to fit the constraints of part one"]
    fn example_two() {
        let input = read_to_string("src/input/day01-test.txt").expect("File could not be read.");

        assert_eq!(part_two(&input), 281);
    }
}
