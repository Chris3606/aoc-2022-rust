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

    pub fn execute(&self, x: i32) -> i32 {
        match self {
            Self::Addx(v) => x + v,
            Self::Noop => x,
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
}

struct CPU<'a, T>
where
    T: Iterator<Item = &'a Instruction>,
{
    x: i32,
    cycle: u32,
    instructions: T,
    current_instruction: ExecutingInstruction,
    finished_program: bool,
}

impl<'a, T> CPU<'a, T>
where
    T: Iterator<Item = &'a Instruction>,
{
    pub fn new(mut instructions: T) -> Self {
        let instr = *instructions.next().unwrap();
        Self {
            x: 1,
            cycle: 0,
            instructions,
            current_instruction: ExecutingInstruction::new(instr),
            finished_program: false,
        }
    }

    /// Completes a cycle of the CPU, returning the value in x _during_ (not after) the current
    /// cycle.
    pub fn tick(&mut self) -> i32 {
        let x = self.x;
        self.cycle += 1;

        self.current_instruction.cycles_remaining -= 1;
        if self.current_instruction.cycles_remaining == 0 {
            self.x = self.current_instruction.instr.execute(self.x);

            if let Some(next) = self.instructions.next() {
                self.current_instruction = ExecutingInstruction::new(*next);
            } else {
                self.finished_program = true;
            }
        }

        x
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut cpu = CPU::new(instructions.iter());

    let mut signal_strength = 0;
    while !cpu.finished_program {
        let x = cpu.tick();
        if cpu.cycle == 20 || cpu.cycle > 20 && (cpu.cycle - 20) % 40 == 0 {
            signal_strength += cpu.cycle as i32 * x
        }
    }

    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut cpu = CPU::new(instructions.iter());
    let mut grid = Grid::new_empty(40, 6, '.');

    let mut pixel_idx = 0;
    while !cpu.finished_program {
        // Tick CPU; the value of x during that tick is the horizontal position of the sprite
        let hpos = cpu.tick();

        // Pixel within the row we're drawing.
        let pixel_col = pixel_idx % grid.width();
        grid[pixel_idx] = if ((hpos - 1)..=(hpos + 1)).contains(&(pixel_col as i32)) {
            '#'
        } else {
            '.'
        };

        pixel_idx = (pixel_idx + 1) % grid.num_cells();
    }

    Some(format!("{}", grid))
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
        let str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(String::from(str)));
    }
}
