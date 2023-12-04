use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = (HashSet<usize>, HashSet<usize>);

// each card has two lists of numbers separated by a vertical bar (|):
// a list of winning numbers and then a list of numbers you have
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut cards = vec![];
    for line in input.lines() {
        // "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        let (winstr, mystr) = line.split_once(" | ").unwrap();

        let (_, winstr) = winstr.split_once(": ").unwrap();
        let winningnums = winstr.split_whitespace()
                                            .map(|x| x.parse().unwrap())
                                            .collect();
        let mynums = mystr.split_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect();                                     

        cards.push((winningnums, mynums))
    }

    Ok(cards)
}

// figure out which of the numbers you have appear in the list of winning numbers.
// The first match makes the card worth one point and each match after the first
// doubles the point value of that card.
// How many points are they worth in total?
#[aoc(day4, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut sum = 0;
    for (winningnums, mynums) in input {
        let mut points = 0;
        for _ in winningnums.intersection(mynums) {
            if points == 0 {
                points = 1;
            }
            else {
                points *= 2;
            }
        }
        sum += points;
    }
    sum
}

// If there are n matches, get m extra copies of the next n cards (where m is the number of copies of this card).
// Including the original set of scratchcards, how many total scratchcards do you end up with?
#[aoc(day4, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut cardcount = vec![1; input.len()];
    for (i, (winningnums, mynums)) in input.into_iter().enumerate() {
        let mycount = cardcount[i];
        for (j, _) in winningnums.intersection(mynums).enumerate() {
            cardcount[i + 1 + j] += mycount;
        }
    }

    cardcount.iter().sum()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 30);
    }
}