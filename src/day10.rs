use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = ((usize, usize), Vec<Vec<char>>);

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut start = (0, 0);
    let mut grid = vec![];
    // First pass: identify find the start
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
            }
            row.push(c);
        }
        grid.push(row);
    }

    Ok((start, grid))
}

#[derive(Eq, PartialEq)]
pub struct Node {
    row: usize,
    col: usize,
    distance: usize,
    c: char
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_loop_neighbors(location: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    match grid[location.0][location.1] {
        'S' => {
            // need to check every direction
            // up
            if location.0 > 0 
                && matches!(grid[location.0 - 1][location.1] ,'|' | '7' | 'F') {
                    neighbors.push((location.0 - 1, location.1))
            }
            // down
            if location.0 < num_rows 
                && matches!(grid[location.0 + 1][location.1] ,'|' | 'L' | 'J') {
                    neighbors.push((location.0 + 1, location.1))
            }
            // left
            if location.1 > 0 
                && matches!(grid[location.0][location.1 - 1] ,'-' | 'L' | 'F') {
                    neighbors.push((location.0, location.1 - 1))
            }
            // right
            if location.1 < num_cols 
                && matches!(grid[location.0][location.1 + 1] ,'-' | '7' | 'J') {
                    neighbors.push((location.0, location.1 + 1))
            }

        },
        '|' => {
            neighbors.push((location.0 - 1, location.1));
            neighbors.push((location.0 + 1, location.1));
        },
        '-' => {
            neighbors.push((location.0, location.1 - 1));
            neighbors.push((location.0, location.1 + 1));},
        'L' => {
            neighbors.push((location.0 - 1, location.1));
            neighbors.push((location.0, location.1 + 1));},
        'J' => {
            neighbors.push((location.0 - 1, location.1));
            neighbors.push((location.0, location.1 - 1));},
        '7' => {
            neighbors.push((location.0, location.1 - 1));
            neighbors.push((location.0 + 1, location.1));},
        'F' => {
            neighbors.push((location.0, location.1 + 1));
            neighbors.push((location.0 + 1, location.1));},
        _ => unreachable!()
    }
    neighbors
}

fn loopfinder(start: &(usize, usize), grid: &Vec<Vec<char>>) -> Vec<Node>{
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    // distance from start to each location
    let mut distance = vec![vec![usize::MAX; num_cols]; num_rows];
    distance[start.0][start.1] = 0;

    let start_node = Node {
        row: start.0,
        col: start.1,
        distance: 0,
        c: grid[start.0][start.1]
    };
    let mut frontier = VecDeque::new();
    frontier.push_back(start_node);
    let mut visited = vec![];

    // Depth-First Search
    // It'd be Breadth-First if this were a Queue instead of a Stack
    // Since I know it's a loop, I think these should the min distances
    while let Some(node) = frontier.pop_front() {
        let neighbors = get_loop_neighbors((node.row, node.col), grid);
        for neighbor in neighbors {
            let new_distance = node.distance + 1;
            if new_distance < distance[neighbor.0][neighbor.1] {
                let neighbor_node = Node {
                    row: neighbor.0, 
                    col: neighbor.1, 
                    distance: new_distance,
                    c: grid[neighbor.0][neighbor.1]
                };
                frontier.push_back(neighbor_node);
                distance[neighbor.0][neighbor.1] = new_distance;
            }
        }
        visited.push(node);
    }

    visited
}

// Max distance in the loop from start
#[aoc(day10, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let (start, grid) = input;
    let mut theloop = loopfinder(start, grid);
    
    // Node cmp compares by distance
    theloop.sort();

    theloop.last().unwrap().distance
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    let (start, grid) = input;
    let mut theloop = loopfinder(start, grid);



    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT1: &'static str =
r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

    const TEST_INPUT2: &'static str =
r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;
    #[test]
    fn test_part1_example1() {
        let input = super::input_generator(TEST_INPUT1).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 4);
    }
    #[test]
    fn test_part1_example2() {
        let input = super::input_generator(TEST_INPUT2).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT1).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}