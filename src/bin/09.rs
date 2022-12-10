use std::{collections::HashSet, str::FromStr};

use advent_of_code::helpers::{manhattan_distance, ParseError, Vector2i, DOWN, LEFT, RIGHT, UP};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Instruction {
    direction: Vector2i,
    amount: u32,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction_data = parts.next().ok_or(ParseError::InvalidInput)?;
        let amount_data = parts.next().ok_or(ParseError::InvalidInput)?;

        let direction = match direction_data {
            "U" => UP,
            "R" => RIGHT,
            "D" => DOWN,
            "L" => LEFT,
            _ => return Err(ParseError::InvalidInput),
        };
        let amount = amount_data
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidInput)?;

        Ok(Self { direction, amount })
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

/// Given the position of a "leader" knot (t he knot in front), calculate the new position of the
/// trailing knot.
fn get_knot_follow_pos(leader_knot: &Vector2i, trailing_knot: &Vector2i) -> Vector2i {
    let dx = leader_knot.x - trailing_knot.x;
    let dy = leader_knot.y - trailing_knot.y;

    let mut delta = Vector2i {
        x: dx.abs(),
        y: dy.abs(),
    };

    match manhattan_distance(&trailing_knot, &leader_knot) {
        3.. => {
            // We have to have to be on different x and y lines.  So we're moving diagonal.
            delta.x = if delta.x >= 2 { 1 } else { delta.x };
            delta.y = if delta.y >= 2 { 1 } else { delta.y };
        }
        2 => {
            // We are either adjacent (1 each axis), or two away on 1.  So we'll just move
            // one closer
            delta.x = if delta.x > 1 { 1 } else { 0 };
            delta.y = if delta.y > 1 { 1 } else { 0 };
        }
        0..=1 => {
            // We're adjacent, so there's nothing to do
            delta = Vector2i { x: 0, y: 0 };
        }
    }

    // We've figured out the magnitude of the move; now ensure we move the right way by
    // making the sign of our delta match the difference
    if dx < 0 {
        delta.x *= -1
    }
    if dy < 0 {
        delta.y *= -1
    }

    *trailing_knot + &delta
}

/// Simulates a rope with the given number of knots (including the "head" and "tail").
///
fn simulate_rope<'a>(num_knots: u32, instructions: impl Iterator<Item = &'a Instruction>) -> usize {
    // Keep track of all rope positions.  We'll assume they all start at (0, 0); the actual positions
    // don't matter (only the relative positions of the knots to each other does) since all we care
    // about is the number of positions the tail visits.
    //
    // 0 will be the head; knots.len() - 1 will be the tail.
    let mut knots = Vec::new();
    for _ in 0..num_knots {
        knots.push(Vector2i { x: 0, y: 0 });
    }

    // Track how many positions the tail visits
    let mut tail_visited = HashSet::new();
    tail_visited.insert(knots[knots.len() - 1]);

    for instr in instructions {
        for _ in 0..instr.amount {
            // Advance the head.
            knots[0] = knots[0] + &instr.direction;

            // Advance every other rope according to the one ahead of it
            for idx in 1..knots.len() {
                knots[idx] = get_knot_follow_pos(&knots[idx - 1], &knots[idx]);
            }

            // Ensure we mark the position the tail is now at appropriately
            tail_visited.insert(knots[knots.len() - 1]);
        }
    }

    tail_visited.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions = parse_input(input);
    let num_positions = simulate_rope(2, instructions.iter());

    Some(num_positions)
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = parse_input(input);
    let num_positions = simulate_rope(10, instructions.iter());

    Some(num_positions)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
