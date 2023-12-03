use std::{collections::HashSet, num::Wrapping};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// 2d array of Values
type Data = Vec<Value>;

pub enum Value {
    Number(char),
    Symbol,
    Period
}

// any number adjacent to a symbol, even diagonally, is a "part number" 
// and should be included in your sum.
// (Periods (.) do not count as a symbol.)
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            if c.is_digit(10) {
                row.push(Value::Number(c));
            }
            else if c == '.' {
                row.push(Value::Period);
            }
            else {
                row.push(Value::Symbol);
            }
        }
        grid.push(row);
    }
    Ok(grid)
}

// how do i get enum type? there's gotta be a better way
fn is_symbol(val: &Value) -> bool {
    match val {
        Value::Symbol => return true,
        _ => return false
    }
}

fn is_number(val: &Value) -> bool {
    match val {
        Value::Number(_) => return true,
        _ => return false
    }
}

fn any_neighbor_symbols(input: &[Data], neighbors: &HashSet<(usize, usize)>) -> bool {
    neighbors.iter().any(|(row, col)| is_symbol(&input[*row][*col]))
} 

// Return all my neighbors' coords
fn my_neighbor_coords(i: usize, j: usize, num_rows: usize, num_cols: usize) -> Vec<(usize, usize)>
{
    let i = Wrapping(i);
    let j = Wrapping(j);
    let one = Wrapping(1);
    let mut candidates = vec![];
    candidates.push((i-one,j-one)); // upleft
    candidates.push((i-one,j)); // up
    candidates.push((i-one,j+one)); // upright
    candidates.push((i,j-one)); // left
    candidates.push((i,j+one)); // right
    candidates.push((i+one,j-one)); // downleft
    candidates.push((i+one,j)); // down
    candidates.push((i+one,j+one)); // downright

    candidates.into_iter().map(|(wrapi, wrapj)| (wrapi.0, wrapj.0))
    // integer wrapping means that if i/j was 1, then i-1/j-1 is now greater than num_rows/cols
    .filter(|(row, col)| {
        *row < num_rows && *col < num_cols
    }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Data]) -> u32 {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> u32 {
    let mut sum = 0;
    let num_rows = input.len();
    let num_cols = input[0].len();
    for (i, row) in input.iter().enumerate() {
        let mut inprogressnum = "".to_string();
        let mut alljs = vec![];
        for (j, entry) in row.iter().enumerate() {
            match entry {
                Value::Number(digit) => {
                    inprogressnum += &digit.to_string();
                    alljs.push(j);
                },
                _ => {
                    if !inprogressnum.is_empty() {
                        // just finished a number, check if it's a part num
                        let mut neighbors = HashSet::new();
                        for &nj in &alljs {
                            neighbors.extend(my_neighbor_coords(i, nj, num_rows, num_cols));
                        }
                        if any_neighbor_symbols(input, &neighbors) {
                            sum += inprogressnum.parse::<u32>().unwrap();
                        }
                        // get ready for the next number
                        inprogressnum = "".to_string();
                        alljs = vec![];
                    }
                }
            }
        }
        // in case the row ended on a number
        let mut neighbors = HashSet::new();
        for &nj in &alljs {
            neighbors.extend(my_neighbor_coords(i, nj, num_rows, num_cols));
        }
        if any_neighbor_symbols(input, &neighbors) {
            sum += inprogressnum.parse::<u32>().unwrap();
        }
    }

    sum
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}