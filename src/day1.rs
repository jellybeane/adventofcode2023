use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = String;

// each line originally contained a specific
// calibration value that the Elves now need to recover.
// On each line, the calibration value can be found by combining the 
// first digit and the last digit (in that order) to form 
// a single two-digit number.
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    // keep them as strings bc parts 1 and 2 have different rules
    let mut result: Vec<String> = vec![];
    for line in input.lines() {
        result.push(line.to_string());
    }

    Ok(result)
}

// What is the sum of all the calibration values?
#[aoc(day1, part1)]
pub fn solve_part1(input: &[Data]) -> u32 {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> u32 {
    let mut calvals: Vec<u32> = vec![];
    for line in input {
        let mut tens: u32 = 0;
        let mut ones: u32 = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if 0 == tens {
                    tens = c.to_digit(10).unwrap() * 10;
                }
                ones = c.to_digit(10).unwrap();
            }
        }
        calvals.push(tens + ones);
    }

    calvals.iter().sum()
}

// Some digits are spelled out with letters
#[aoc(day1, part2)]
pub fn solve_part2(input: &[Data]) -> u32 {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> u32 {
    let mut calvals: Vec<u32> = vec![];
    for line in input {
        let mut tens: u32 = 0;
        let mut ones: u32 = 0;
        let mut maybespelled = "".to_owned();
        for c in line.chars() {
            if c.is_ascii_digit() {
                if 0 == tens {
                    tens = c.to_digit(10).unwrap() * 10;
                }
                ones = c.to_digit(10).unwrap();
            }
            else {
                {
                    maybespelled = format!("{}{}", maybespelled, c);
                    let maybenum = spelledout(&maybespelled);
                    if maybenum != 0 {
                        if 0 == tens {
                            tens = maybenum * 10;
                        }
                        ones = maybenum;
                        // last/first letter might be shared, eg "eightwo"
                        maybespelled = maybespelled.chars().last().unwrap().to_string();
                    }
                    
                }
            }
        }
        calvals.push(tens + ones);
    }

    calvals.iter().sum()
}

const SPELLING: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn spelledout(maybenum: &str) -> u32 {
    for (spelling, number) in SPELLING {
        if maybenum.contains(spelling) {
            return number
        }
    }

    0
}

#[cfg(test)]
mod test {
    const TEST_INPUT1: &'static str =
r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
const TEST_INPUT2: &'static str =
r#"two1nine
eightwothree
eightwo
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
treb7uchet
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT1).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT2).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 281 + 82 + 77);
    }
}