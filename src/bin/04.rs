use std::str::FromStr;

use advent_of_code::helpers::ParseError;

/// Represents a number range with a min and a max value.
struct NumRange {
    min: u32,
    max: u32,
}

impl FromStr for NumRange {
    type Err = ParseError;

    /// Parses a number range in the format {min}-{max}
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_it = s.split('-');
        let min = num_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidInput)?;
        let max = num_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidInput)?;

        Ok(NumRange { min, max })
    }
}

impl NumRange {
    /// Whether or not the given range overlaps with the current one.
    pub fn overlaps(&self, other: &NumRange) -> bool {
        self.min <= other.max && other.min <= self.max
    }

    /// Whether or not the given range is completely contained within the current one.
    pub fn contains(&self, other: &NumRange) -> bool {
        self.min <= other.min && other.max <= self.max
    }
}

/// Represents a pair or ranges, one for each elf, as per the input.
struct RangePair {
    elf1_range: NumRange,
    elf2_range: NumRange,
}

impl FromStr for RangePair {
    type Err = ParseError;

    /// Parses a RangePair in the form {NumRange from_str format},{NumRange from_str format}
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_it = s.split(',');
        let elf1_range = num_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<NumRange>()
            .map_err(|_| ParseError::InvalidInput)?;
        let elf2_range = num_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<NumRange>()
            .map_err(|_| ParseError::InvalidInput)?;

        Ok(RangePair {
            elf1_range,
            elf2_range,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let fully_overlapping_ranges = input
        .lines()
        .map(|l| l.parse::<RangePair>().unwrap())
        .filter(|p| p.elf1_range.contains(&p.elf2_range) || p.elf2_range.contains(&p.elf1_range))
        .count();

    Some(fully_overlapping_ranges)
}

pub fn part_two(input: &str) -> Option<usize> {
    let overlapping_ranges = input
        .lines()
        .map(|l| l.parse::<RangePair>().unwrap())
        .filter(|p| p.elf1_range.overlaps(&p.elf2_range))
        .count();

    Some(overlapping_ranges)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
