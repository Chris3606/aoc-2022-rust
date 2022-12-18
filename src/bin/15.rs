use std::{collections::HashMap, str::FromStr};

use advent_of_code::helpers::{ParseError, Vector2i};

struct MapData {
    sensors: Vec<Vector2i>,
    beacons: Vec<Vector2i>,
    dists: HashMap<Vector2i, u64>,
}

impl FromStr for MapData {
    type Err = ParseError;

    pub fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
