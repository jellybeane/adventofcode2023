use std::{cmp::Ordering, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// each hand and bid
type Data = Hand;
#[derive(PartialEq, Eq, Clone,Debug)]
pub struct Hand {
    cards: Vec<u32>,
    bid: usize,
    handtype: HandType
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Ordering::*;
        let result = self.handtype
                    .cmp(&other.handtype);
        match result {
            // if they have the same hand type, compare each card in order
            Equal => {
                for (i, mycard) in self.cards.iter().enumerate() {
                    let othercard = other.cards[i];
                    let cardresult = mycard.cmp(&othercard);
                    if Equal != cardresult {
                        return Some(cardresult);
                    }
                }
                Some(Equal)
            },
            _ => Some(result)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap() // ???
    }
}

pub fn hand_type(cards: &Vec<u32>) -> HandType {
    use HandType::*;

    let mut counter = HashMap::new();
    for card in cards {
        match counter.get(card) {
            Some(count) => counter.insert(card, count + 1),
            None => counter.insert(card, 1),
        };
    }
    match counter.len() {
        5 => HighCard,
        4 => OnePair,
        3 => {
            for (_, count) in counter {
                if count == 3 {
                    return Three;
                }
            }
            TwoPair
        },
        2 => {
            for (_, count) in counter {
                if count == 4 {
                    return Four;
                }
            }
            FullHouse
        },
        1 => Five,
        _ => unreachable!()
    }
}

// Just ordering by type: not checking the card value
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone,Debug)]
pub enum HandType {
    // default ordering has increasing value going down
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse, 
    Four,
    Five
}

// Each line is a hand and a bid
// 32T3K 765
#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut allhands = vec![];
    for line in input.lines() {
        let (handstr, bidstr) = line.split_once(" ").unwrap();

        let mut cards = vec![];
        for c in handstr.chars() {
            let card = if c.is_numeric() {
                c.to_digit(10).unwrap()
            }
            else {
                match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14, // aces high
                    _ => unreachable!()
                }
            };
            cards.push(card);
        }
        let bid = bidstr.parse().unwrap();
        let handtype = hand_type(&cards);
        allhands.push(Hand { cards, bid, handtype });
    }

    Ok(allhands)
}

// Each hand wins an amount equal to its bid multiplied by its rank
// What are the total winnings?
#[aoc(day7, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut allhands = input.to_vec();
    allhands.sort();
    let mut sum = 0;
    for (i, hand) in allhands.iter().enumerate() {
        //dbg!(hand);
        sum += (i+1) * hand.bid;
    }
    sum
}

// J cards are now Jokers that can join any other to make the strongest type
// J cards are now the weakest, below 2.
#[aoc(day7, part2)]
pub fn solve_part2(input: &[Data]) -> u32 {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::day7::HandType;

    use super::Hand;

    const TEST_INPUT: &'static str =
r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_Three_Twopair() {
        use crate::day7::hand_type;
        // T55J5
        let threecards: Vec<u32> = vec![10, 5, 5, 11, 5];
        let handtype = hand_type(&threecards);
        let three = Hand {cards:threecards, bid:684, handtype};
        // KK677
        let twopaircards = vec![13, 13, 6, 7, 7];
        let handtype = hand_type(&twopaircards);
        let two = Hand {cards:twopaircards, bid:28, handtype};

        assert_eq!(three.handtype, HandType::Three);
        assert_eq!(two.handtype, HandType::TwoPair);
        
        // Three of a kind stronger than two pair
        assert_eq!(HandType::Three.cmp(&HandType::TwoPair), Ordering::Greater);
        assert_eq!(HandType::TwoPair.cmp(&HandType::Three), Ordering::Less);
        assert_eq!(three.cmp(&two), Ordering::Greater);
        assert_eq!(two.cmp(&three), Ordering::Less);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 5905);
    }
}