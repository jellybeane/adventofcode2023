use std::collections::{VecDeque, HashSet};

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

// copy-pasting last year's Day12
#[derive(Eq, PartialEq, Hash)]
pub struct Node {
    row: usize,
    col: usize,
    distance: usize
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

fn bfs<F>(start: &(usize, usize), grid: &Vec<Vec<char>>, get_neighbors: F) 
    -> Vec<Node>
    where F: Fn((usize, usize), &Vec<Vec<char>>) -> Vec<(usize, usize)>
{
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    // distance from start to each location
    let mut distance = vec![vec![usize::MAX; num_cols]; num_rows];
    distance[start.0][start.1] = 0;

    let start_node = Node {
        row: start.0,
        col: start.1,
        distance: 0
    };
    let mut frontier = VecDeque::new();
    frontier.push_back(start_node);
    let mut visited = vec![];

    // Breadth-First Search: frontier is a stack
    // Since I know it's a loop, these should the min distances
    while let Some(node) = frontier.pop_front() {
        let neighbors = get_neighbors((node.row, node.col), grid);
        for neighbor in neighbors {
            let new_distance = node.distance + 1;
            if new_distance < distance[neighbor.0][neighbor.1] {
                let neighbor_node = Node {
                    row: neighbor.0, 
                    col: neighbor.1, 
                    distance: new_distance
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
    let mut theloop = bfs(start, grid, get_loop_neighbors);
    
    // Node cmp compares by distance
    theloop.sort();

    theloop.last().unwrap().distance
}

// all neighbors that aren't in the loop
fn get_floodfill_neighbors(location: (usize, usize), grid: &Vec<Vec<char>>, theloop: &[Node]) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;

    let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    for offset in offsets {
        // needs to be isize in order to check for -1
        let row = (location.0 as isize) + offset.0;
        let col = (location.1 as isize) + offset.1;
        // does the neighbor actually exist, and is it not a loop member?
        if row >= 0 && row < num_rows && col >= 0 && col < num_cols 
                && !theloop.iter()
                    .any(|n| n.row as isize == row && n.col as isize == col) {
            neighbors.push((row as usize, col as usize));
        }
    }
    neighbors
}

fn floodfill(grid: &Vec<Vec<char>>, theloop: &[Node]) -> HashSet<Node> {
    // making the assumption that the top left corner is not part of the loop
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    // distance from start to each location
    let mut distance = vec![vec![usize::MAX; num_cols]; num_rows];
    distance[0][0] = 0;

    let start_node = Node {
        row: 0,
        col: 0,
        distance: 0
    };
    let mut frontier = VecDeque::new();
    frontier.push_back(start_node);
    let mut visited = HashSet::new();

    // Breadth-First Search
    while let Some(node) = frontier.pop_front() {
        let neighbors = get_floodfill_neighbors((node.row, node.col), grid, theloop);
        for neighbor in neighbors {
            let new_distance = node.distance + 1;
            if new_distance < distance[neighbor.0][neighbor.1] {
                let neighbor_node = Node {
                    row: neighbor.0, 
                    col: neighbor.1, 
                    distance: new_distance
                };
                frontier.push_back(neighbor_node);
                distance[neighbor.0][neighbor.1] = new_distance;
            }
        }
        visited.insert(node);
    }

    visited
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    let (start, grid) = input;
    let theloop = bfs(start, grid, get_loop_neighbors);
    // oops get_floodfill_neighbors takes loop as an arg
    //let outside = bfs(start, grid, get_floodfill_neighbors);
    let outside = floodfill(grid, &theloop);

    // if it's not part of the loop or outside, it's inside the loop
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let totalcells = num_rows * num_cols;

    totalcells - theloop.len() - outside.len()
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

    const TEST_INPUT3: &'static str =
r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
    #[test]
    fn test_part2_example1() {
        let input = super::input_generator(TEST_INPUT3).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 4);
    }

    const TEST_INPUT4: &'static str =
r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
    #[test]
    fn test_part2_example2() {
        let input = super::input_generator(TEST_INPUT4).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 10);
    }
}