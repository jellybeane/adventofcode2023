use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<isize>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut parsed = vec![];
    for line in input.lines() {
        let numbers: Vec<isize> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        parsed.push(numbers);
    }
    Ok(parsed)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Data]) -> isize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> isize {
    let mut sum = 0;
    for history in input {
        sum += predict_next(history);
    }

    sum
}

fn predict_next(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|&x| x == 0) {
        return 0;
    }

    let diffs = sequence.windows(2)
                .map(|pair| pair[1] - pair[0]).collect();
    
    sequence.last().unwrap() + predict_next(&diffs)
}

fn predict_prev(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|&x| x == 0) {
        return 0;
    }

    let diffs = sequence.windows(2)
                .map(|pair| pair[1] - pair[0]).collect();
    
    sequence[0] - predict_prev(&diffs)
}


#[aoc(day9, part2)]
pub fn solve_part2(input: &[Data]) -> isize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> isize {
    let mut sum = 0;
    for history in input {
        sum += predict_prev(history);
    }

    sum
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 114);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 2);
    }
}