//! --- Day 1: Trebuchet?! ---

// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

// For example:

// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of the calibration values?

use std::fs::read_to_string;

#[test]
fn test_calculate() {
    let input: Vec<String> = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    assert_eq!(calculate(input), 142);
}

fn calculate(input: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in input {
        let digits: Vec<char> = line
            .chars()
            .filter(|c| c.to_string().parse::<u8>().is_ok())
            .collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let value = format!("{}{}", first, last).parse::<u32>().unwrap();
        sum += value;
    }
    sum
}

fn main() {
    let res = calculate(
        read_to_string("inputs/day1a.txt")
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect(),
    );
    println!("Result: {}", res);
    // cor r ect an s we r is 5 4 5 8 1
}
