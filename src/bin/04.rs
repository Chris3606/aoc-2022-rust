use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

/// Represents a number range with a min and a max value.
struct NumRange {
    min: u32,
    max: u32,
}

/// Parses a number range in the format {min}-{max}
fn parse_range(input: &str) -> IResult<&str, NumRange> {
    let (input, (min, max)) = separated_pair(
        nom::character::complete::u32,
        tag("-"),
        nom::character::complete::u32,
    )(input)?;

    Ok((input, NumRange { min, max }))
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

/// Parses a RangePair in the form {NumRange from_str format},{NumRange from_str format}
fn parse_range_pair(input: &str) -> IResult<&str, RangePair> {
    let (input, (elf1_range, elf2_range)) =
        separated_pair(parse_range, tag(","), parse_range)(input)?;

    Ok((
        input,
        RangePair {
            elf1_range,
            elf2_range,
        },
    ))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, ranges) = separated_list1(newline, parse_range_pair)(input).unwrap();
    let fully_overlapping_ranges = ranges
        .iter()
        .filter(|p| p.elf1_range.contains(&p.elf2_range) || p.elf2_range.contains(&p.elf1_range))
        .count();

    Some(fully_overlapping_ranges)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, ranges) = separated_list1(newline, parse_range_pair)(input).unwrap();
    let overlapping_ranges = ranges
        .iter()
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
