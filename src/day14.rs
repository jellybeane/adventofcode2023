use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<Vec<char>>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut pattern = vec![];
    for line in input.lines(){
        pattern.push(line.chars().collect());
    }
    Ok(pattern)
}

fn rolledrocks_after_tilt(pattern: &[Vec<char>]) -> Vec<(usize, usize)> {
    // first pass: original positions
    let mut fixedrocks = vec![];
    let mut roundrocks = vec![];
    for (i, row) in pattern.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            match c {
                '#' => fixedrocks.push((i, j)),
                'O' => roundrocks.push((i, j)),
                _ => {}
            }
        }
    }
    
    // tilt north
    let mut rolledrocks = vec![];
    for (rrow, rcol) in roundrocks {
        let mut newrow = 0;
        for &(frow, fcol) in &fixedrocks {
            if fcol == rcol {
                assert!(frow != rrow);
                if frow < rrow && frow >= newrow {
                    newrow = frow + 1;
                }
            }
        }
        fixedrocks.push((newrow, rcol));
        rolledrocks.push((newrow, rcol));
    }

    rolledrocks
}

fn calculate_load(roundrocks: Vec<(usize, usize)>, numrows: usize) -> usize {
    roundrocks.iter()
        .map(|(row, _)| numrows - row)
            .sum()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let roundrocks = rolledrocks_after_tilt(input);
    calculate_load(roundrocks, input.len())
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 136);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}