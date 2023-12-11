use std::cmp::{min, max};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<Vec<char>>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut rows = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        rows.push(row);
    }

    Ok(rows)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    solve_everything(input, 2)
}

fn find_empty(original: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = vec![];
    let mut empty_columns = vec![];
    for (i, row) in original.iter().enumerate() {
        if row.iter().all(|&c| '.' == c) {
            empty_rows.push(i);
        }
    }

    let original_numcols = original[0].len();
    for j in 0..original_numcols {
        let mut is_empty = true;
        for row in original {
            if '#' == row[j] {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_columns.push(j);
        }
    }

    (empty_rows, empty_columns)
}

fn expanded_manhattan(a: (usize, usize), b:(usize, usize),
    empty_rows: &[usize], empty_cols: &[usize], expansion: usize) -> usize
{
    // don't double-count the already-existing empty row/column
    let expansion = expansion - 1;

    let rowmin = min(a.0, b.0);
    let rowmax = max(a.0, b.0);
    let colmin = min(a.1, b.1);
    let colmax = max(a.1, b.1);
    
    let mut rowdistance = rowmax - rowmin;
    for i in rowmin..rowmax {
        if empty_rows.contains(&i) {
            rowdistance += expansion;
        }
    }
    
    let mut coldistance = colmax - colmin;
    for j in colmin..colmax {
        if empty_cols.contains(&j) {
            coldistance += expansion;
        }
    }

    rowdistance + coldistance
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    solve_everything(input, 1000000)
}

fn solve_everything(input: &Data, expansion: usize) -> usize {
    let (empty_rows, empty_columns) = find_empty(input);
    
    let mut galaxies = vec![];
    for (i, row) in input.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if '#' == c {
                galaxies.push((i, j));
            }
        }
    }

    // distance from every galaxy to every other galaxy
    let mut sum = 0;
    for g1 in 0..(galaxies.len()-1) {
        for g2 in (g1+1)..galaxies.len() {
            let galaxy1 = galaxies[g1];
            let galaxy2 = galaxies[g2];
            let distance = expanded_manhattan(galaxy1, galaxy2, &empty_rows, &empty_columns, expansion);

            sum += distance;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 374);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_everything(&input, 10);

        assert_eq!(result, 1030);
    }
}