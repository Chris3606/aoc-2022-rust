use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, IResult};

type ElfInventory = Vec<u32>;

fn parse_inventory(input: &str) -> IResult<&str, ElfInventory> {
    let (input, inventory) = separated_list1(newline, nom::character::complete::u32)(input)?;
    Ok((input, inventory))
}

fn parse_input(input: &str) -> IResult<&str, Vec<ElfInventory>> {
    let (input, elf_inventories) = separated_list1(tag("\n\n"), parse_inventory)(input)?;

    Ok((input, elf_inventories))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, elfs) = parse_input(input).unwrap();
    let most_calories = elfs.iter().map(|i| i.iter().sum()).max().unwrap();

    Some(most_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, elfs) = parse_input(input).unwrap();
    let mut calories: Vec<_> = elfs.iter().map(|i| i.iter().sum()).collect();
    calories.sort();

    let top_three_calories = calories.iter().rev().take(3).sum();
    Some(top_three_calories)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
