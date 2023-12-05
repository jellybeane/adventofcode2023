use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// the list of seeds, and map of sources to transition maps
type Data = (Vec<isize>, HashMap<Type, TransitionMap>);

#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TransitionMap {
    source: Type,
    dest: Type,
    source_starts: Vec<isize>,
    dest_starts: Vec<isize>,
    ranges: Vec<isize>
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut lines: VecDeque<&str> = input.lines().collect();
    // line 1: "seeds: 79 14 55 13"
    let (_, seedline) = lines.pop_front().unwrap().split_once(": ").unwrap();
    let seeds: Vec<isize> = seedline
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
    let mut transitionmaps = HashMap::new();
    
    let mut source = Type::Seed;
    let mut dest = Type::Seed;
    let mut source_starts = vec![];
    let mut dest_starts = vec![];
    let mut ranges = vec![];
    for line in lines {
        if line.is_empty() {
            // finished a transition map
            transitionmaps.insert(source, TransitionMap {source, dest, source_starts, dest_starts, ranges});
            // reset for the next loop
            source_starts = vec![];
            dest_starts = vec![];
            ranges = vec![];
        }
        else if line.chars().next().unwrap().is_numeric() {
            // dest range start, source range start, and range length
            let numbers: Vec<isize> = line.split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();
            dest_starts.push(numbers[0]);
            source_starts.push(numbers[1]);
            ranges.push(numbers[2]);
        }
        else {
            // it's the Source-to-Dest line
            // janky thing for now
            match line {
                "seed-to-soil map:" => {
                    source = Type::Seed;
                    dest = Type::Soil;
                },
                "soil-to-fertilizer map:" => {
                    source = Type::Soil;
                    dest = Type::Fertilizer;
                },
                "fertilizer-to-water map:" => {
                    source = Type::Fertilizer;
                    dest = Type::Water;
                },
                "water-to-light map:" => {
                    source = Type::Water;
                    dest = Type::Light;
                },
                "light-to-temperature map:" => {
                    source = Type::Light;
                    dest = Type::Temperature;
                },
                "temperature-to-humidity map:" => {
                    source = Type::Temperature;
                    dest = Type::Humidity;
                },
                "humidity-to-location map:" => {
                    source = Type::Humidity;
                    dest = Type::Location;
                }
                _ => unreachable!()
            }
        }
    }
    // in case there wasn't a newline at the end
    if !(source_starts.is_empty()) {
        transitionmaps.insert(source, TransitionMap {source, dest, source_starts, dest_starts, ranges});
    }

    Ok((seeds, transitionmaps))
}

// find the lowest location number that corresponds to any of the initial seeds
// you'll need to convert each seed number through other categories until you can find its corresponding location number
#[aoc(day5, part1)]
pub fn solve_part1(input: &Data) -> isize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> isize {
    let (seeds, transitionmaps) = input;
    let mut lowest_location = isize::MAX;

    for &seed in seeds {
        let mut sourcetype = Type::Seed;
        let mut transitionmap;
        let mut desttype;
        let mut sourceval = seed;
        let mut destval = sourceval;
        // go through the transition maps until we get to Location
        while sourcetype != Type::Location {
            transitionmap = transitionmaps.get(&sourcetype).unwrap();
            desttype = transitionmap.dest;
            dbg!(desttype);
            destval = sourceval;
            // check each transition range because i didn't implement intervals
            for (i, sourcestart) in transitionmap.source_starts.iter().enumerate() {
                let offset:isize = sourceval - sourcestart;
                if offset >= 0 && offset < transitionmap.ranges[i] {
                    destval = transitionmap.dest_starts[i] + offset;
                    break;
                }
            }
            // update for the next loop
            sourcetype = desttype;
            //dbg!(sourcetype);
            sourceval = destval;
        }
        if destval < lowest_location {
            lowest_location = destval;
        }
    }
    
    lowest_location
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}