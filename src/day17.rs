use std::collections::{VecDeque, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<Vec<usize>>;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut array = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let number: usize = c.to_digit(10).unwrap() as usize;
            row.push(number);
        }
        array.push(row);
    }
    Ok(array)
}

fn find_path(grid: &Vec<Vec<usize>>) -> HashMap<(usize, usize), (usize, usize)>{
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut distance = vec![vec![usize::MAX; num_cols]; num_rows];
    distance[0][0] = 0;

    let mut frontier = VecDeque::new();
    frontier.push_back((0,0));

    let mut prev = HashMap::new();

    while let Some(node) = frontier.pop_front() {
        let neighbors = get_neighbors(&node, grid, &prev);
        for neighbor in neighbors {
            let new_distance = distance[node.0][node.1] + grid[neighbor.0][neighbor.1];
            //dbg!(new_distance);
            if new_distance < distance[neighbor.0][neighbor.1] {
                frontier.push_back(neighbor);
                distance[neighbor.0][neighbor.1] = new_distance;
                prev.insert(neighbor, node);
            }
        }
        
    }

    dbg!(distance[num_rows-1][num_cols-1]);

    prev
}

fn get_neighbors(location:&(usize, usize), grid: &Vec<Vec<usize>>, prev: &HashMap<(usize, usize), (usize, usize)>)
 -> Vec<(usize, usize)> {
    use Direction::*;
    let mut neighbors = vec![];
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    
    let mut directions = HashSet::new();
    directions.insert(Up);
    directions.insert(Down);
    directions.insert(Left);
    directions.insert(Right);
    if let Some(prev_loc) = prev.get(location) {
        let dir1 = get_direction(prev_loc, location);
        // cannot reverse direction
        let forbidden = match dir1 {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        };
        directions.remove(&forbidden);
        
        // can only go up to 3 blocks in the same direction
        if let Some(prev_prev_loc) = prev.get(prev_loc) {
            let dir2 = get_direction(prev_prev_loc, prev_loc);
                if dir1 == dir2 {
                    directions.remove(&dir2);
                }
        }
    }

    let row = location.0;
    let col = location.1;
    for dir in directions {
        match dir {
            Up => if row > 0 { neighbors.push((row - 1, col)); }
            Down => if row < num_rows - 1 { neighbors.push((row + 1, col)); },
            Left => if col > 0 { neighbors.push((row, col - 1)); },
            Right => if col < num_cols - 1 { neighbors.push((row, col + 1)); },
        }
    }

    neighbors
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Up, 
    Down,
    Left, 
    Right
}

// the direction of b relative to a
fn get_direction(a: &(usize, usize), b: &(usize, usize)) -> Direction{
    if b.0 < a.0 {
        return Direction::Up;
    }
    else if b.0 > a.0 {
        return Direction::Down;
    }
    else if b.1 < a.1 {
        return Direction::Left;
    }
    else if b.1 > a.1 {
        return Direction::Right;
    }
    unreachable!()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let prevmap = find_path(input);

    let mut toprint = vec![vec!['.'; num_cols]; num_rows];
    let mut sum = 0;
    let mut block = (num_rows - 1, num_cols - 1);
    toprint[num_rows - 1][num_cols - 1] = '#';
    while let Some(&prev_block) = prevmap.get(&block) {
        toprint[prev_block.0][prev_block.1] = '#';
        sum += input[block.0][block.1];
        block = prev_block;
    }

    for row in toprint {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    sum
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 102);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}