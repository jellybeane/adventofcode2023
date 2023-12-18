use std::{vec, collections::{HashSet, VecDeque}};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = (Direction, usize, String);

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Up, 
    Down,
    Left, 
    Right
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut plan = vec![];
    for line in input.lines() {
        let split: Vec<_> = line.split(' ').collect();
        let dirstr = split[0];
        let numstr = split[1];
        let colorstr = split[2];

        let direction = match dirstr {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!()
        };
        let num: usize = numstr.parse().unwrap();
        plan.push((direction, num, colorstr.to_owned()));
    }

    Ok(plan)
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut path = HashSet::new();
    let mut pos: (isize, isize) = (0, 0);
    path.insert(pos);
    for &(direction, num, _) in input {
        for _ in 0..num {
            pos = match direction {
                Direction::Up => (pos.0-1, pos.1),
                Direction::Down => (pos.0+1, pos.1),
                Direction::Left => (pos.0, pos.1-1),
                Direction::Right => (pos.0, pos.1+1),
            };
            path.insert(pos);
        }
    }

    let row_max = path.iter().map(|(row, _)| row).max().unwrap();
    let row_min = path.iter().map(|(row, _)| row).min().unwrap();
    let col_max = path.iter().map(|(_, col)| col).max().unwrap();
    let col_min = path.iter().map(|(_, col)| col).min().unwrap();
    dbg!(row_min, row_max, col_min, col_max);

    let num_rows = (row_max - row_min + 1) as usize;
    let num_cols = (col_max - col_min + 1) as usize;
    dbg!(num_rows, num_cols);
    let mut grid = vec![vec!['.'; num_cols]; num_rows];
    for coords in &path {
        let converted = convert_coords(coords, row_min, col_min);
        grid[converted.0][converted.1] = '#';
    }

    let converted0 = convert_coords(&(0,0), row_min, col_min);
    dbg!(converted0);
    grid[converted0.0][converted0.1] = 'O';

    // for (i, row) in grid.iter().enumerate() {
    //     for (j,c) in row.iter().enumerate() {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let path: Vec<_> = path.iter().map(|coords| convert_coords(coords, row_min, col_min)).collect();
    // from looking, I see that (1,1) is inside
    let start = convert_coords(&(1,1), row_min, col_min);
    let filled = floodfill(start, num_rows, num_cols, &path);

    path.len() + filled.len()
}

fn convert_coords(coords:&(isize, isize), row_min: &isize, col_min: &isize) -> (usize, usize) {
    assert!(coords.0 >= *row_min);
    assert!(coords.1 >= *col_min);
    let row = (coords.0 - row_min) as usize;
    let col = (coords.1 - col_min) as usize;

    (row, col)
}

// all neighbors that aren't in the loop
fn get_floodfill_neighbors(location: (usize, usize), num_rows: usize, num_cols: usize, path: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    let num_rows = num_rows as isize;
    let num_cols = num_cols as isize;

    let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    for offset in offsets {
        // needs to be isize in order to check for -1
        let row = (location.0 as isize) + offset.0;
        let col = (location.1 as isize) + offset.1;
        // does the neighbor actually exist, and is it not a loop member?
        if row >= 0 && row < num_rows && col >= 0 && col < num_cols 
                && !path.iter()
                    .any(|&(nrow, ncol)| nrow as isize == row && ncol as isize == col) {
            neighbors.push((row as usize, col as usize));
        }
    }
    neighbors
}

fn floodfill(start: (usize, usize), num_rows: usize, num_cols: usize, path: &[(usize, usize)]) -> HashSet<(usize, usize)> {
    // distance from start to each location
    let mut distance = vec![vec![usize::MAX; num_cols]; num_rows];
    distance[start.0][start.0] = 0;

    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    let mut visited = HashSet::new();

    // Breadth-First Search
    while let Some(node) = frontier.pop_front() {
        let neighbors = get_floodfill_neighbors((node.0, node.1), num_rows, num_cols, path);
        for neighbor in neighbors {
            let new_distance = distance[node.0][node.1] + 1;
            if new_distance < distance[neighbor.0][neighbor.1] {
                frontier.push_back(neighbor);
                distance[neighbor.0][neighbor.1] = new_distance;
            }
        }
        visited.insert(node);
    }

    visited
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 62);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}