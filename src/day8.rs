use std::collections::{VecDeque, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;
use ::num::Integer;

type Data = (String, HashMap<String, (String, String)>);

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut lines: VecDeque<&str> = input.lines().collect();

    // first line: left/right instructions
    let instructions = lines.pop_front().unwrap().to_owned();
    // an empty line
    lines.pop_front();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in lines.into_iter() {
        // AAA = (BBB, CCC)
        let (thisnode, nextnodes) = line.split_once(" = ").unwrap();
        let nextnodes = nextnodes.replace("(", "");
        let nextnodes = nextnodes.replace(")", "");
        let (leftnode, rightnode) = nextnodes.split_once(", ").unwrap();
        nodes.insert(thisnode.to_owned(), (leftnode.to_owned(), rightnode.to_owned()));
    }

    Ok((instructions, nodes))
}

// How many steps are required to reach ZZZ?
#[aoc(day8, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let (instructionstr, nodes) = input;
    let instructions = instructionstr.as_bytes();

    let mut curnode: &String = &"AAA".to_string();
    let mut steps = 0;
    while "ZZZ" != curnode {
        let (left, right) = nodes.get(curnode).unwrap();
        curnode = match instructions[steps % instructions.len()] {
            b'L' => {left},
            b'R' => {right},
            _ => unreachable!()
        };
        steps += 1;
    }
    steps
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    let (instructionstr, nodes) = input;
    let instructions = instructionstr.as_bytes();

    let startnodes: Vec<&String> = nodes.keys().filter(|x| x.ends_with("A")).collect();
    let mut cycle_lengths = vec![];
    for startnode in startnodes {
        let mut steps = 0;
        let mut curnode = startnode;
        while !curnode.ends_with("Z") {
            let (left, right) = nodes.get(curnode).unwrap();
            curnode = match instructions[steps % instructions.len()] {
                b'L' => {left},
                b'R' => {right},
                _ => unreachable!()
            };
            steps += 1;
        }
        cycle_lengths.push(steps);
    }

    least_common_multiple(cycle_lengths)
}

fn least_common_multiple(numbers: Vec<usize>) -> usize {
    let mut l:usize = 1;
    for n in numbers {
        l = l.lcm(&n);
    }
    l
}

#[cfg(test)]
mod test {
    const TEST_INPUT1: &'static str =
r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
const TEST_INPUT2: &'static str =
r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT1).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT2).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 6);
    }
}