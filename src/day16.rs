use std::collections::{VecDeque, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<Vec<char>>;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut grid = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    Ok(grid)
}

#[derive(Eq, PartialEq, Hash)]
enum Direction {
    Top, Bottom, Left, Right
}

#[derive(Eq, PartialEq, Hash)]
pub struct PathTile {
    row: usize,
    col: usize,
    entry: Direction // the direction entered from
}

fn next_step(row: usize, col: usize, dir: Direction, num_rows: usize, num_cols: usize) -> Option<PathTile> {
    use Direction::*;
    
    let coords = match dir {
        Top => if row < num_rows -1 {
             Some((row + 1, col) )
        } else {
            None
        },
        Bottom => if row > 0 {
             Some((row - 1, col) )
        } else {
            None
        } ,
        Left => if col < num_cols -1 {
             Some((row, col + 1) )
        } else {
            None
        } ,
        Right => if col > 0 {
             Some((row, col - 1) )
        } else {
            None
        } ,
    };

    if let Some((row, col)) = coords {
        Some(PathTile { row, col, entry: dir })
    }
    else {
        None
    }
    
}

fn get_neighbors(me: &PathTile, grid: &Vec<Vec<char>>) -> Vec<PathTile>{
    use Direction::*;
    let mut neighbors = vec![];
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    match grid[me.row][me.col] {
        '.' => {
            match me.entry {
                Top => neighbors.push(next_step(me.row, me.col, Top, num_rows, num_cols)) ,
                Bottom => neighbors.push(next_step(me.row, me.col, Bottom, num_rows, num_cols)),
                Left => neighbors.push(next_step(me.row, me.col, Left, num_rows, num_cols)),
                Right => neighbors.push(next_step(me.row, me.col, Right, num_rows, num_cols)),
            }
        },
        '/' => {
            match me.entry {
                Top => neighbors.push(next_step(me.row, me.col, Right, num_rows, num_cols)),
                Bottom => neighbors.push(next_step(me.row, me.col, Left, num_rows, num_cols)),
                Left => neighbors.push(next_step(me.row, me.col, Bottom, num_rows, num_cols)),
                Right => neighbors.push(next_step(me.row, me.col, Top, num_rows, num_cols)),
            }
        },
        '\\' => {
            match me.entry {
                Top => neighbors.push(next_step(me.row, me.col, Left, num_rows, num_cols)),
                Bottom => neighbors.push(next_step(me.row, me.col, Right, num_rows, num_cols)),
                Left => neighbors.push(next_step(me.row, me.col, Top, num_rows, num_cols)),
                Right => neighbors.push(next_step(me.row, me.col, Bottom, num_rows, num_cols)),
            }
        },
        '-' => {
            match me.entry {
                Top | Bottom => {
                    neighbors.push(next_step(me.row, me.col, Right, num_rows, num_cols));
                    neighbors.push(next_step(me.row, me.col, Left, num_rows, num_cols));
                },
                Left => neighbors.push(next_step(me.row, me.col, Left, num_rows, num_cols)),
                Right => neighbors.push(next_step(me.row, me.col, Right, num_rows, num_cols)),
            }
        },
        '|' => {
            match me.entry {
                Top => neighbors.push(next_step(me.row, me.col, Top, num_rows, num_cols)),
                Bottom => neighbors.push(next_step(me.row, me.col, Bottom, num_rows, num_cols)),
                Left | Right => {
                    neighbors.push(next_step(me.row, me.col, Top, num_rows, num_cols));
                    neighbors.push(next_step(me.row, me.col, Bottom, num_rows, num_cols));
                },
            }
        },
        _ => unreachable!()
    }

    let mut actualneighbors = vec![];
    for neighbor in neighbors.into_iter().flatten() {
        actualneighbors.push(neighbor);
    }
    actualneighbors
}

fn count_energized(start_node: PathTile, grid: &Vec<Vec<char>>) -> usize{
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut frontier = VecDeque::new();
    frontier.push_back(start_node);

    let mut visited = HashSet::new();
    let mut energized = vec![vec!['.';num_cols];num_rows];

    while let Some(node) = frontier.pop_front() {
        let neighbors = get_neighbors(&node, grid);
        for neighbor in neighbors {
            // only add neighbor if we have not entered that tile from that direction
            if !visited.contains(&neighbor) {
                frontier.push_back(neighbor);
            }
        }
        energized[node.row][node.col] = '#';
        visited.insert(node);
    }

    let mut count = 0;
    for row in energized {
        for c in row {
            if c == '#' {
                count += 1;
            }
            //print!("{}", c);
        }
        //println!();
    }
    count
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let start_node = PathTile {
        row: 0,
        col: 0,
        entry: Direction::Left
    };
    count_energized(start_node, input)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    let mut best = 0;
    let num_rows = input.len();
    let num_cols = input[0].len();

    // top
    for col in 0..num_cols-1 {
        let start_node = PathTile {
            row: 0,
            col,
            entry: Direction::Top
        };
        let energized = count_energized(start_node, input);
        if energized > best {
            best = energized;
        }
    }
    // bottom
    for col in 0..num_cols-1 {
        let start_node = PathTile {
            row: num_rows - 1,
            col,
            entry: Direction::Bottom
        };
        let energized = count_energized(start_node, input);
        if energized > best {
            best = energized;
        }
    }
    // left
    for row in 0..num_rows-1 {
        let start_node = PathTile {
            row,
            col: 0,
            entry: Direction::Left
        };
        let energized = count_energized(start_node, input);
        if energized > best {
            best = energized;
        }
    }
    // right
    for row in 0..num_rows-1 {
        let start_node = PathTile {
            row,
            col: num_cols - 1,
            entry: Direction::Right
        };
        let energized = count_energized(start_node, input);
        if energized > best {
            best = energized;
        }
    }

    best
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 46);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 51);
    }
}