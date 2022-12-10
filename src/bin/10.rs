use std::str::FromStr;

use advent_of_code::helpers::{Grid, ParseError};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts_it = s.split(' ');
        let instr_data = parts_it.next().ok_or(ParseError::InvalidInput)?;
        let instr = match instr_data {
            "addx" => {
                let num = parts_it
                    .next()
                    .ok_or(ParseError::InvalidInput)?
                    .parse::<i32>()
                    .map_err(|_| ParseError::InvalidInput)?;
                Instruction::Addx(num)
            }
            "noop" => Instruction::Noop,
            _ => return Err(ParseError::InvalidInput),
        };

        Ok(instr)
    }
}

impl Instruction {
    pub fn num_cycles(&self) -> u32 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }

    pub fn execute(&self, cpu: &mut CPU) {
        match self {
            Self::Addx(v) => cpu.x += v,
            Self::Noop => {}
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ExecutingInstruction {
    instr: Instruction,
    cycles_remaining: u32,
}

impl ExecutingInstruction {
    pub fn new(instr: Instruction) -> Self {
        ExecutingInstruction {
            instr: instr,
            cycles_remaining: instr.num_cycles(),
        }
    }

    pub fn tick(&mut self, cpu: &mut CPU) -> bool {
        self.cycles_remaining -= 1;

        if self.cycles_remaining == 0 {
            self.instr.execute(cpu);
            return true;
        }

        false
    }
}

struct CPU {
    x: i32,
    cycle: u32,
}

impl CPU {
    pub fn new() -> Self {
        Self { x: 1, cycle: 0 }
    }

    pub fn execute_program(&mut self, program: &Vec<Instruction>) -> i32 {
        let mut signal_strength = 0;

        let mut instr_it = program.iter();
        let mut current_instr = ExecutingInstruction::new(*instr_it.next().unwrap());
        loop {
            // The question asks for the value _during_ the given cycles, not _after_
            // those cycles complete; so we must check before we complete an instruction
            // this cycle
            self.cycle += 1;
            if self.cycle == 20 || self.cycle > 20 && (self.cycle - 20) % 40 == 0 {
                signal_strength += self.cycle as i32 * self.x
            }

            let finished = current_instr.tick(self);
            if finished {
                if let Some(next) = instr_it.next() {
                    current_instr = ExecutingInstruction::new(*next);
                } else {
                    break;
                }
            }
        }

        signal_strength
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut cpu = CPU::new();
    let signal_strength = cpu.execute_program(&instructions);

    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut cpu = CPU::new();

    let mut grid = Grid::new_empty(40, 6, ' ');

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
