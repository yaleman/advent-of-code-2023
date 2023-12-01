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

use advent_of_code_2023::Digit;

#[test]
fn test_calculate() {
    let input: Vec<String> = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    assert_eq!(calculate_day1b(input), 281);
}

fn calculate_day1b(input: Vec<impl ToString>) -> u32 {
    let mut sum = 0;

    let known_digits = Digit::digits();

    for line in input {
        let line = line.to_string();
        let mut first: Option<u8> = None;
        let mut last: Option<u8> = None;
        println!("Line: {}", line);
        for i in 0..=line.len() - 1 {
            // get the chars starting at index i
            let linechars = &line[i..];
            println!("- {}", linechars);

            if first.is_none() {
                if let Ok(vl) = linechars.chars().next().unwrap().to_string().parse::<u8>() {
                    first = Some(vl);
                    println!("First: {:?}", first);
                } else {
                    for digit in &known_digits {
                        if linechars.starts_with(&(digit.to_string())) {
                            first = Some(digit.into());
                        }
                    }
                }
            }
            if let Ok(vl) = linechars.chars().next().unwrap().to_string().parse::<u8>() {
                last = Some(vl);
                println!("Last: {:?}", last);
            } else {
                for digit in &known_digits {
                    if linechars.starts_with(&(digit.to_string())) {
                        last = Some(digit.into());
                        println!("Last: {:?}", last);
                        break;
                    } else {
                        // println!("{} didn't match {}", linechars, &(digit.to_string()));
                    }
                }
            }
        }
        println!("First: {:?} Last: {:?}", first, last);
        let value = format!("{}{}", first.unwrap(), last.unwrap())
            .parse::<u32>()
            .unwrap();
        sum += value;
    }
    sum
}

fn main() {
    let res = calculate_day1b(
        read_to_string("inputs/day1a.txt")
            .unwrap()
            .lines()
            .collect(),
    );
    println!("Result: {}", res);
}

#[test]
fn test_calculate_72eightwoh() {
    let res = calculate_day1b(vec!["72eightwoh".to_string()]);
    assert_eq!(res, 72);
}
