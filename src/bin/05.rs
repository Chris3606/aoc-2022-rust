use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, IResult};
use std::str::FromStr;

use advent_of_code::helpers::ParseError;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl FromStr for Stacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = Vec::new();

        let mut input = s.lines().rev();
        _ = input.next().ok_or(ParseError::InvalidInput)?; // Eat the line of numbers; we don't need it

        for line in input {
            for (stack_idx, value_idx) in (1..line.len()).step_by(4).enumerate() {
                if stack_idx >= stacks.len() {
                    stacks.push(Vec::new());
                }
                let ch = line
                    .chars()
                    .nth(value_idx)
                    .ok_or(ParseError::InvalidInput)?;
                if ch != ' ' {
                    let stack = &mut stacks[stack_idx];
                    stack.push(ch);
                }
            }
        }

        Ok(Self { stacks })
    }
}

impl Stacks {
    pub fn execute_9000(&mut self, instr: &Instruction) {
        for _ in 0..instr.num {
            let ch = self.stacks[instr.from - 1].pop().unwrap();
            self.stacks[instr.to - 1].push(ch);
        }
    }

    pub fn execute_9001(&mut self, instr: &Instruction) {
        assert!(instr.from != instr.to);
        let from = instr.from - 1;
        let to = instr.to - 1;
        let from_len = self.stacks[from].len();

        for idx in (from_len - instr.num)..from_len {
            let ch = self.stacks[from][idx];
            self.stacks[to].push(ch);
        }

        for _ in 0..instr.num {
            self.stacks[from].pop();
        }
    }

    pub fn get_message(&self) -> String {
        let mut str = String::new();
        for s in &self.stacks {
            str.push(*s.last().unwrap());
        }

        str
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, num) = nom::character::complete::u32(input)?;

    let (input, _) = tag(" from ")(input)?;
    let (input, from) = nom::character::complete::u32(input)?;

    let (input, _) = tag(" to ")(input)?;
    let (input, to) = nom::character::complete::u32(input)?;

    Ok((
        input,
        Instruction {
            num: num as usize,
            from: from as usize,
            to: to as usize,
        },
    ))
}

pub fn part_one(input: &str) -> Option<String> {
    let mut parts_it = input.split("\n\n");
    let stack_data = parts_it.next().unwrap();

    let mut stacks = stack_data.parse::<Stacks>().unwrap();
    let (_, instructions) =
        separated_list1(newline, parse_instruction)(parts_it.next().unwrap()).unwrap();

    for i in &instructions {
        stacks.execute_9000(i);
    }

    let message = stacks.get_message();
    Some(message)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut parts_it = input.split("\n\n");
    let stack_data = parts_it.next().unwrap();

    let mut stacks = stack_data.parse::<Stacks>().unwrap();
    let (_, instructions) =
        separated_list1(newline, parse_instruction)(parts_it.next().unwrap()).unwrap();

    for i in &instructions {
        stacks.execute_9001(i);
    }

    let message = stacks.get_message();
    Some(message)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
