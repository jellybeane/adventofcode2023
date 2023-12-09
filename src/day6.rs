use std::iter::zip;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = (String, String);

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
// Time:      7  15   30
// Distance:  9  40  200
fn input_generator_inner(input: &str) -> Result<Data> {
    let lines: Vec<&str> = input.lines().collect();
    let (_, timestr) = lines[0].split_once(": ").unwrap();
    let (_, diststr) = lines[1].split_once(": ").unwrap();

    Ok((timestr.to_owned(), diststr.to_owned()))
}

//determine the number of ways you can beat the record
//What do you get if you multiply these numbers together?
#[aoc(day6, part1)]
pub fn solve_part1(input: &Data) -> isize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> isize {
    let (timestr, diststr) = input;
    // individual races separated by whitespace
    let times: Vec<isize> = timestr.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect();  
    let distances: Vec<isize> = diststr.split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();  

    let mut product = 1;
    for (t, d) in zip(times, distances)
    {
        product *= num_ways_to_beat(t, d);
    }
    product
}

fn num_ways_to_beat(t: isize, d: isize) -> isize {
    // want integer h where
    // d < (t - h) * h
    let mut maxh = 10;
    let mut minh = 5;
    // brute force intersect finding lol
    for h in 1..t {
        if d < (t - h) * h {
            minh = h;
            break;
        }
    }
    for h in (1..t).rev() {
        if d < (t - h) * h {
            maxh = h;
            break;
        }
    }
    maxh - minh + 1
}

// There's just one race with excess whitespace
#[aoc(day6, part2)]
pub fn solve_part2(input: &Data) -> isize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> isize {
    let (timestr, diststr) = input;
    // want something like python's string strip
    let t: isize = timestr.replace(' ', "").parse().unwrap();
    let d: isize = diststr.replace(' ', "").parse().unwrap();
    dbg!(t, d);
    num_ways_to_beat(t, d)
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"Time:      7  15   30
Distance:  9  40  200
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 71503);
    }
}