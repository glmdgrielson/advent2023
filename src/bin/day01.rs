/// Day one of Advent of Code
use std::fs::read_to_string;

/// Find the first and last digits of each line and add them.
fn part_one(data: &str) -> usize {
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

fn part_two(data: &str) -> usize {
    let mut sum = 0;

    for line in data.lines() {
        let (one, two) = find_digit(line.to_string());

        let total = one * 10 + two;
        sum += total as usize;
    }

    return sum;
}

fn find_digit(line: String) -> (usize, usize) {
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

    // Find the number closest to the right.
    let search_r = numbers
        .iter()
        .filter_map(|s| match line.rfind(s) {
            Some(idx) => Some((s, idx)),
            None => None,
        })
        .max_by_key(|(_, idx)| idx.clone());

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
                .unwrap() as usize
        } else {
            number
        }
    } else {
        // This line _has_ no text digit to find
        // so we run the same check from part one.
        line.chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found") as usize
    };

    let two = if let Some((number, index)) = search_r {
        let number = text_to_number(number);

        // We know every line has a digit in it, because
        // part one requires it, so this mess is technically
        // okay to do.
        //
        // Unfortunately, since we have to put in the call to
        // `.rev` here, this index needs `nth_back` to find.
        let digit_idx = line.chars().rev().position(|c| c.is_ascii_digit()).unwrap();

        // Check if this is the latest number.
        if digit_idx > index {
            // The digit came later, so we need to turn the
            // index into a digit.

            // We know that this index exists because we literally just found it.
            let ch = line.chars().nth_back(digit_idx).unwrap();

            // We know that this is a digit due to the test we made
            // to find this index.
            ch.to_digit(10).unwrap() as usize
        } else {
            number
        }
    } else {
        // This line has no text digit to find,
        // so we run the same check from part one.
        line.chars()
            .rfind(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .expect("Digit not found") as usize
    };
    (one, two)
}

/// Turn a textual number into a digit.
fn text_to_number(text: &str) -> usize {
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
    let input = read_to_string("input/day01.txt").expect("File could not be read.");

    println!("Part one solution is: {}", part_one(&input));

    assert_eq!(find_digit("eightwo1three".to_string()), (8, 3));

    println!("Part two solution is: {}", part_two(&input));
    // let mut lines = input.lines();
    // print!("{}", lines.next().expect("Missing data"));
    //
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_two() {
        let input = read_to_string("input/day01-test.txt").expect("File could not be read.");

        assert_eq!(part_two(&input), 281);
    }
}
