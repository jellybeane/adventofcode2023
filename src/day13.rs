use std::cmp::min;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<Vec<char>>;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut patterns = vec![];
    let mut pattern = vec![];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = vec![];
        }
        else {
            pattern.push(line.chars().collect());
        }
    }
    patterns.push(pattern);

    Ok(patterns)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut sum = 0;
    for pattern in input {
        sum += column_reflect(pattern) + 100*row_reflect(pattern);
    }
    sum
}

fn column_reflect(pattern: &Vec<Vec<char>>) -> usize {
    let numcols = pattern[0].len();
    for j in 0..numcols-1 {
        let mut is_reflect = true;
        let numcompares = min(j, (numcols-1) - (j+1));
        for step in 0..=numcompares {
            let left = j - step;
            let right = j+1 + step;
            for row in pattern {
                if row[left] != row[right] {
                    is_reflect = false;
                    break;
                }
                if !is_reflect {
                    break;
                }
            }
        }
        if is_reflect {
            return j+1;
        }
    }
    0
}

fn row_reflect(pattern: &Vec<Vec<char>>) -> usize {
    let numrows = pattern.len();
    for i in 0..numrows-1 {
        let mut is_reflect = true;
        let numcompares = min(i, (numrows-1) - (i+1));
        for step in 0..=numcompares {
            let up = i - step;
            let down = i+1 + step;
            if pattern[up] != pattern[down] {
                is_reflect = false;
                break;
            }
        }
        if is_reflect {
            return i+1;
        }
    }
    0
}


#[aoc(day13, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut sum = 0;
    for pattern in input {
        sum += column_reflect2(pattern) + 100*row_reflect2(pattern);
    }
    sum
}

fn column_reflect2(pattern: &Vec<Vec<char>>) -> usize {
    let numcols = pattern[0].len();
    for j in 0..numcols-1 {
        let mut numdiffs = 0;
        let numcompares = min(j, (numcols-1) - (j+1));
        for step in 0..=numcompares {
            let left = j - step;
            let right = j+1 + step;
            for row in pattern {
                if row[left] != row[right] {
                    numdiffs += 1;
                }
            }
        }
        if numdiffs == 1 {
            return j+1; 
        }
    }
    0
}

fn row_reflect2(pattern: &Vec<Vec<char>>) -> usize {
    let numrows = pattern.len();
    let numcols = pattern[0].len();
    for i in 0..numrows-1 {
        let mut numdiffs = 0;
        let numcompares = min(i, (numrows-1) - (i+1));
        for step in 0..=numcompares {
            let up = i - step;
            let down = i+1 + step;
            for j in 0..numcols {
                if pattern[up][j] != pattern[down][j] {
                    numdiffs += 1;
                }
            }
        }
        if numdiffs == 1 {
            return i+1; 
        }
    }
    0
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 405);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 400);
    }
}