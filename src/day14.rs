use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = RockPattern;

pub struct RockPattern {
    cube: Vec<(usize, usize)>,
    round: Vec<(usize, usize)>,
    numrows: usize,
    numcols: usize
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut pattern = vec![];
    for line in input.lines(){
        pattern.push(line.chars().collect());
    }

    Ok(original_rocks(&pattern))
}

fn original_rocks(pattern: &[Vec<char>]) 
    -> RockPattern {
    let mut cuberocks = vec![];
    let mut roundrocks = vec![];
    for (i, row) in pattern.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            match c {
                '#' => cuberocks.push((i, j)),
                'O' => roundrocks.push((i, j)),
                _ => {}
            }
        }
    }
    RockPattern { cube: cuberocks, round: roundrocks, numrows: pattern.len(), numcols: pattern[0].len() }
}

fn tilt_north(cuberocks: &[(usize, usize)], roundrocks: &[(usize, usize)])
    -> Vec<(usize, usize)> {
    let mut fixedrocks = Vec::from(cuberocks);
    let mut roundrocks = Vec::from(roundrocks);
    // resolve in increasing row order
    roundrocks.sort_by(|a, b| a.0.cmp(&b.0));

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

fn tilt_west(cuberocks: &[(usize, usize)], roundrocks: &[(usize, usize)])
    -> Vec<(usize, usize)> {
    let mut fixedrocks = Vec::from(cuberocks);
    let mut roundrocks = Vec::from(roundrocks);
    // resolve in increasing column order
    roundrocks.sort_by(|a, b| a.1.cmp(&b.1));

    let mut rolledrocks = vec![];
    for (rrow, rcol) in roundrocks {
        let mut newcol = 0;
        for &(frow, fcol) in &fixedrocks {
            if frow == rrow {
                assert!(fcol != rcol);
                if fcol < rcol && fcol >= newcol {
                    newcol = fcol + 1;
                }
            }
        }
        fixedrocks.push((rrow, newcol));
        rolledrocks.push((rrow, newcol));
    }

    rolledrocks
}

fn tilt_south(cuberocks: &[(usize, usize)], roundrocks: &[(usize, usize)], numrows: usize)
    -> Vec<(usize, usize)> {
    let mut fixedrocks = Vec::from(cuberocks);
    let mut roundrocks = Vec::from(roundrocks);
    // resolve in decreasing row order
    roundrocks.sort_by(|a, b| b.0.cmp(&a.0));

    let mut rolledrocks = vec![];
    for (rrow, rcol) in roundrocks {
        let mut newrow = numrows - 1;
        for &(frow, fcol) in &fixedrocks {
            if fcol == rcol {
                assert!(frow != rrow);
                if frow > rrow && frow <= newrow {
                    newrow = frow - 1;
                }
            }
        }
        fixedrocks.push((newrow, rcol));
        rolledrocks.push((newrow, rcol));
    }

    rolledrocks
}

fn tilt_east(cuberocks: &[(usize, usize)], roundrocks: &[(usize, usize)], numcols: usize)
    -> Vec<(usize, usize)> {
    let mut fixedrocks = Vec::from(cuberocks);
    let mut roundrocks = Vec::from(roundrocks);
    // resolve in decreasing column order
    roundrocks.sort_by(|a, b| b.1.cmp(&a.1));

    let mut rolledrocks = vec![];
    for (rrow, rcol) in roundrocks {
        let mut newcol = numcols - 1;
        for &(frow, fcol) in &fixedrocks {
            if frow == rrow {
                assert!(fcol != rcol);
                if fcol > rcol && fcol <= newcol {
                    newcol = fcol - 1;
                }
            }
        }
        fixedrocks.push((rrow, newcol));
        rolledrocks.push((rrow, newcol));
    }

    rolledrocks
}

fn calculate_load(roundrocks: &[(usize, usize)], numrows: usize) -> usize {
    roundrocks.iter()
        .map(|(row, _)| numrows - row)
            .sum()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let rolled = tilt_north(&input.cube, &input.round);
    calculate_load(&rolled, input.numrows)
}

fn spin_cycle(cuberocks: &[(usize, usize)], roundrocks: &[(usize, usize)], numrows:usize, numcols: usize) 
    -> Vec<(usize, usize)>
{
    let mut round = Vec::from(roundrocks);

    round = tilt_north(cuberocks, &round);
    round = tilt_west(cuberocks, &round);
    round = tilt_south(cuberocks, &round, numrows);
    round = tilt_east(cuberocks, &round, numcols);

    round
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    let mut round = input.round.clone();
    let mut seen_states = vec![];
    let mut index = 0;
    let mut cycle_offset = 0;
    let mut cycle_length = 1000000000;
    'outer:
    for _ in 0..1000000000 {
        seen_states.push(round.to_vec());
        round = spin_cycle(&input.cube, &round, input.numrows, input.numcols);
        index += 1;
        for (i, state) in seen_states.iter().enumerate() {
            if state == &round {
                cycle_offset = i;
                cycle_length = index - i;
                break 'outer;
            }
        }
    }
    
    #[cfg(test)]
    {
        dbg!(cycle_offset, cycle_length, index);
        for (i, state) in seen_states.iter().enumerate() {
            println!("{}", i);
            prettyprint(&input.cube, state, input.numrows, input.numcols);
            println!("{}", calculate_load(state, input.numrows));
            println!();
        }
    }

    let final_loop_index = (1000000000 - cycle_offset) % cycle_length;
    calculate_load(&seen_states[final_loop_index + cycle_offset], input.numrows)
}

#[cfg(test)]
fn prettyprint(cuberocks: &[(usize, usize)], roundrocks: &[(usize, usize)], numrows: usize , numcols: usize) {
    let mut grid = vec![vec!['.';numcols];numrows];
    for &(i, j) in cuberocks {
        grid[i][j] = '#';
    }
    for &(i, j) in roundrocks {
        grid[i][j] = 'O';
    }
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
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

        assert_eq!(result, 64);
    }

    #[test]
    fn part2_rolling() {
        use super::*;
        let input = input_generator(TEST_INPUT).unwrap();
        let mut round = input.round.clone();
        
        println!("After 1 cycle");
        round = spin_cycle(&input.cube, &round, input.numrows, input.numcols);
        prettyprint(&input.cube, &round, input.numrows, input.numcols);
        println!();

        println!("After 2 cycles");
        round = spin_cycle(&input.cube, &round, input.numrows, input.numcols);
        prettyprint(&input.cube, &round, input.numrows, input.numcols);
        println!();
        
        println!("After 3 cycles");
        round = spin_cycle(&input.cube, &round, input.numrows, input.numcols);
        prettyprint(&input.cube, &round, input.numrows, input.numcols);
    }
}