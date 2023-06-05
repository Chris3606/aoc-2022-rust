use std::{collections::HashSet, str::FromStr};

use advent_of_code::helpers::ParseError;

struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
    items: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err(ParseError::InvalidInput);
        }
        let half = s.len() / 2;

        let c1 = &s[0..half];
        let c2 = &s[half..];

        Ok(Rucksack {
            compartment1: c1.chars().collect(),
            compartment2: c2.chars().collect(),
            items: s.chars().collect(),
        })
    }
}

fn score_item(item: char) -> u32 {
    if item.is_lowercase() {
        (item as u32) - ('a' as u32) + 1
    } else {
        (item as u32) - ('A' as u32) + 27
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks: Vec<_> = input
        .lines()
        .map(|l| l.parse::<Rucksack>().unwrap())
        .collect();

    let score = rucksacks
        .iter()
        .map(|r| r.compartment1.intersection(&r.compartment2).next().unwrap())
        .map(|i| score_item(*i))
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<_> = input
        .lines()
        .map(|l| l.parse::<Rucksack>().unwrap())
        .collect();

    let mut sum = 0;
    for sacks in rucksacks.chunks(3) {
        let (i1, i2, i3) = (&sacks[0], &sacks[1], &sacks[2]);

        let mut item = None;
        for c in &i1.items {
            if i2.items.contains(&c) && i3.items.contains(&c) {
                item = Some(*c);
                break;
            }
        }

        sum += score_item(item.unwrap());
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
