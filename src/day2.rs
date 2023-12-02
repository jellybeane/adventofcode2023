use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// Vec of (red, green, blue)
type Data = Vec<(usize, usize, usize)>;
// eg "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// For each game:
//    For each subset (semicolon delimited)
//        Red, Green, Blue cube (comma delimited)
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut result: Vec<Data> = vec![];

    for line in input.lines(){
        let mut gamecubes: Data = vec![];

        let (_, subsets_str) = line.split_once(": ").unwrap();
        let subsets: Vec<_> = subsets_str.split("; ").collect();
        for subset in subsets {
            let subsetsplit = subset.split(", ");
            let mut red: usize = 0;
            let mut blue: usize = 0;
            let mut green: usize = 0;
            for cubestr in subsetsplit {
                let (cube_count_str, color_str) = cubestr.split_once(" ").unwrap();
                let cube_count = cube_count_str.parse().unwrap();
                match color_str {
                    "red" => red = cube_count,
                    "blue" => blue = cube_count,
                    "green" => green = cube_count,
                    _ => unreachable!(),
                }
            }
            gamecubes.push((red, green, blue));
        }
        result.push(gamecubes);
    }

    Ok(result)
}

// Determine which games would have been possible if the bag had been loaded
// with only 12 red cubes, 13 green cubes, and 14 blue cubes. 
// What is the sum of the IDs of those games?
#[aoc(day2, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut sum = 0;
    for (i, game) in input.iter().enumerate() {
        let mut impossible = false;
        for &(red, green, blue) in game {
            if red > 12 || green > 13 || blue > 14 {
                impossible = true;
                break;
            }
        }
        if !impossible {
            // Game IDs are 1 indexed
            sum += i + 1
        }
    }
    sum
}

// what is the fewest number of cubes of each color 
// that could have been in the bag to make the game possible?
// The power of a set of cubes is equal to the numbers of 
// red, green, and blue cubes multiplied together.
// What is the sum of the power of the minimum sets?
#[aoc(day2, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut powers = vec![];
    for game in input {
        let mut redmax = 0;
        let mut greenmax = 0;
        let mut bluemax = 0;
        for &(red, green, blue) in game {
            redmax = redmax.max(red);
            greenmax = greenmax.max(green);
            bluemax = bluemax.max(blue);
        }
        powers.push(redmax * greenmax * bluemax);
    }
    powers.iter().sum()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 2286);
    }
}