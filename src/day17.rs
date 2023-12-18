use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};

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
    use Direction::*;
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut cost = HashMap::new();
    cost.insert(Up, vec![vec![usize::MAX; num_cols]; num_rows]);
    cost.insert(Down, vec![vec![usize::MAX; num_cols]; num_rows]);
    cost.insert(Left, vec![vec![usize::MAX; num_cols]; num_rows]);
    cost.insert(Right, vec![vec![usize::MAX; num_cols]; num_rows]);
    for costgrid in cost.values_mut() {
        costgrid[0][0] = 0;
    }

    // Dijkstra: frontier is a Priority Queue
    // Tuples are compared lexicographically, so store (cost, (row, len))
    let mut queue = BinaryHeap::new();
    queue.push((0, (0,0)));

    let mut prev = HashMap::new();
    
    while let Some((mycost, node)) = queue.pop() {
        //dbg!(mycost, node);

        let neighbors = get_neighbors(&node, grid, &prev);
        for (dir, neighbor) in neighbors {
            let new_cost = (-mycost as usize) + grid[neighbor.0][neighbor.1];
            //dbg!(new_cost);
            if new_cost < cost.get(&dir).unwrap()[neighbor.0][neighbor.1] {
                // binary heaps are max-heaps, so put the negative cost to make it act like a min-heap
                queue.push((-(new_cost as isize), neighbor));
                cost.get_mut(&dir).unwrap()[neighbor.0][neighbor.1] = new_cost;
                prev.insert(neighbor, node);
            }
        }
    }

    for (_, costgrid) in cost {
        dbg!(costgrid[num_rows - 1][num_cols - 1]);
    }

    prev
}

fn get_neighbors(location:&(usize, usize), grid: &Vec<Vec<usize>>, prev: &HashMap<(usize, usize), (usize, usize)>)
 -> Vec<(Direction, (usize, usize))> {
    use Direction::*;
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
            if let Some(prev_prev_prev_loc) = prev.get(prev_prev_loc) {
                let dir3 = get_direction(prev_prev_prev_loc, prev_prev_loc);
                let dir2 = get_direction(prev_prev_loc, prev_loc);
                if dir1 == dir2 && dir2 == dir3 {
                    directions.remove(&dir2);
                }
            }
        }
    }

    let mut neighbors = vec![];
    let row = location.0;
    let col = location.1;
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    for dir in directions {
        match dir {
            Up => if row > 0 { neighbors.push((Up, (row - 1, col))); }
            Down => if row < num_rows - 1 { neighbors.push((Down, (row + 1, col))); },
            Left => if col > 0 { neighbors.push((Left, (row, col - 1))); },
            Right => if col < num_cols - 1 { neighbors.push((Right, (row, col + 1))); },
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

    let mut toprint = vec![vec!['.'; num_cols]; num_rows];
    toprint[num_rows - 1][num_cols - 1] = '#';

    for (node, prev) in prevmap {
        let dir = get_direction(&node, &prev);
        let c = match dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        toprint[node.0][node.1] = c;
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

    const TEST_INPUT2: &'static str =
r#"111119
999199
999111
"#;
    #[test]
    fn test_small() {
        let input = super::input_generator(TEST_INPUT2).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}