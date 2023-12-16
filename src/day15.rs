use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = String;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut steps = vec![];
    for line in input.lines() {
        steps.extend(line.split(',').map(|x| x.to_owned()));
    }
    Ok(steps)
}

fn holiday_ascii_string_helper(s: &str) -> usize {
    let mut value = 0;
    for &c in s.as_bytes() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }

    value
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    input.iter().map(|s| holiday_ascii_string_helper(s)).sum()
}

struct Step {
    label: String,
    op: char,
    focal_length: usize 
}

fn handle_step(s: &str) -> Step {
    let mut label = "".to_owned();
    let mut op = '?';
    let mut focal_length_str = "".to_owned();
    for c in s.chars() {
        if c.is_ascii_digit() {
            focal_length_str.push(c);
        }
        else if c == '-' {
            op = c;
            focal_length_str = "0".to_owned();
        }
        else if c == '=' {
            op = c;
        }
        else {
            label.push(c);
        }
    }

    let focal_length = focal_length_str.parse().unwrap();
    Step { label, op, focal_length }
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut boxes: HashMap<usize, Vec<(usize, String)>> = HashMap::new();

    for s in input {
        let step = handle_step(s);
        let boxnum = holiday_ascii_string_helper(&step.label);
        let lensbox = boxes.entry(boxnum).or_insert_with(Vec::new);
        let mut index = None;
            for (i, (_, label)) in lensbox.iter().enumerate() {
                if *label == step.label {
                    index = Some(i);
                }
            }
        if step.op == '-' {
            if let Some(i) = index {
                lensbox.remove(i);
            }
        }
        else {
            match index {
                Some(i) => lensbox[i] = (step.focal_length, step.label),
                None => lensbox.push((step.focal_length, step.label)),
            };
        }
    }

    let mut sum = 0;
    for (boxnum, lensbox) in boxes {
        for (slot_index, &(focal_length, _)) in lensbox.iter().enumerate() {
            sum += focus_power(boxnum, slot_index, focal_length);
        }
    }

    sum
}

fn focus_power(boxnum: usize, slot_index: usize, focal_length:usize) -> usize {
    let mut power = 1 + boxnum;
    power *= 1 + slot_index;
    power *= focal_length;

    power
}

#[cfg(test)]
mod test {
    use crate::day15::holiday_ascii_string_helper;

    const TEST_INPUT: &'static str =
r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 1320);
    }

    #[test]
    fn test_holiday_ascii_string_helper() {
        assert_eq!(holiday_ascii_string_helper("HASH"), 52);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 145);
    }
}