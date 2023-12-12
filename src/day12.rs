use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = (String, Vec<usize>);

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut records = vec![];
    for line in input.lines() {
        let (rowstr, numstr) = line.split_once(' ').unwrap();

        let nums = numstr.split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        records.push((rowstr.to_owned(), nums));
    }

    Ok(records)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    input.iter()
        .map(|(rowstr, nums)| num_arrangements(&rowstr.chars().collect::<Vec<_>>(), nums))
        .sum()
}

fn num_arrangements(row: &[char], nums: &[usize]) -> usize {
    let totalcharcount = row.len();
    let mut state = vec![HashMap::new(); totalcharcount + 1];
    state[0].insert((row, nums), 1);

    for i in 0..totalcharcount {
        let mut subproblem_results = HashMap::new();
        std::mem::swap(&mut subproblem_results, &mut state[i]);
        for ((row, nums), result) in subproblem_results {
            // add broken
            let next_group = nums.first();
            if let Some(&group_size) = next_group {
                if can_add_broken(row, group_size) {
                    let consume = if group_size == row.len() {
                        group_size
                    }
                    else {
                        group_size + 1
                    };
                    let toupdate = &mut state[i+consume];
                    let newrow = &row[consume..];
                    let newnums = &nums[1..];
                    toupdate.entry((newrow, newnums))
                        .and_modify(|ways| *ways += result)
                        .or_insert(result);
                }
            }
            
            // add operational
            if can_add_operational(row) {
                // TODO: bounds checking
                let toupdate = &mut state[i+1];
                let newrow = &row[1..];
                let newnums = nums;
                toupdate.entry((newrow, newnums))
                    .and_modify(|ways| *ways += result)
                    .or_insert(result);
            }
        }
    }

    let mut total = 0;
    // solutions for the entire string that satisfied all nums
    for ((row, nums), result) in state.last().unwrap() {
        if row.is_empty() && nums.is_empty() {
            total += result;
        }
    }
    total
}

fn can_add_broken(row: &[char], group_size: usize) -> bool {
    if group_size > row.len() {
        return false;
    }

    for &c in &row[0..group_size] {
        if c == '.' {
            return false;
        }
    }

    if group_size < row.len() {
        row[group_size] != '#'
    }
    else {
        true
    }
}
fn can_add_operational(row: &[char]) -> bool{
    !row.is_empty() && row[0] != '#'
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut bigger = vec![];
    for (rowstr, nums) in input {
        let bigstr = (1..5)
            .fold(rowstr.to_owned(), |b: String, _| b + "?" + rowstr);
        let bignums = (1..5)
            .fold(nums.to_owned(), |mut b, _| {b.extend(nums); b});
        bigger.push((bigstr, bignums));
    }

    bigger.iter()
        .map(|(rowstr, nums)| num_arrangements(&rowstr.chars().collect::<Vec<_>>(), nums))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day12::num_arrangements;

    const TEST_INPUT: &'static str =
r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 21);
    }
    
    #[test]
    fn test_num_arrangements() {
        let rowstr = "???.###".chars().collect::<Vec<_>>();
        let nums = vec![1,1,3];
        assert_eq!(num_arrangements(&rowstr, &nums), 1);
        
        let rowstr = ".??..??...?##.".chars().collect::<Vec<_>>();
        let nums = vec![1,1,3];
        assert_eq!(num_arrangements(&rowstr, &nums), 4);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 525152);
    }
}