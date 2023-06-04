use advent_of_code::helpers::ParseError;
use std::str::FromStr;

/// Represents a choice made by either a player or their opponent during a round of
/// rock-paper-scissors.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Choice {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Choice {
    /// Returns what choice this one wins against.
    pub fn beats(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    /// Returns what choice this one will lose to.
    pub fn beat_by(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
}

/// Represents a single round in rock-paper-scissors.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Round {
    yours: Choice,
    opponents: Choice,
}

/// Represents the result of a given rock-paper-scissors round.
enum RoundResult {
    Win,
    Loss,
    Draw,
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts_it = s.split(" ");
        let opponents = match parts_it.next().ok_or(ParseError::InvalidInput)? {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => return Err(ParseError::InvalidInput),
        };

        let yours = match parts_it.next().ok_or(ParseError::InvalidInput)? {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => return Err(ParseError::InvalidInput),
        };

        Ok(Round { yours, opponents })
    }
}

impl Round {
    /// Gets this round's result, from the player's perspective.
    pub fn get_result(&self) -> RoundResult {
        if self.yours == self.opponents {
            return RoundResult::Draw;
        }
        if self.opponents == self.yours.beats() {
            RoundResult::Win
        } else {
            RoundResult::Loss
        }
    }

    /// Scores this round according to the described algorithm.
    pub fn get_score(&self) -> u32 {
        let play_score = match self.yours {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        let result_score = match self.get_result() {
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        };

        play_score + result_score
    }
}

/// Represents a line from the strategy guide, as defined in part 2.
struct StrategyGuideData {
    opponents_move: Choice,
    required_result: RoundResult,
}

impl FromStr for StrategyGuideData {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts_it = s.split(" ");
        let opponents_move = match parts_it.next().ok_or(ParseError::InvalidInput)? {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => return Err(ParseError::InvalidInput),
        };

        let required_result = match parts_it.next().ok_or(ParseError::InvalidInput)? {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => return Err(ParseError::InvalidInput),
        };

        Ok(StrategyGuideData {
            opponents_move,
            required_result,
        })
    }
}

impl StrategyGuideData {
    /// Gets the move the player should make in order to comply with this strategy entry.
    pub fn get_correct_move(&self) -> Round {
        let your_choice = match self.required_result {
            RoundResult::Win => self.opponents_move.beat_by(),
            RoundResult::Draw => self.opponents_move,
            RoundResult::Loss => self.opponents_move.beats(),
        };

        Round {
            opponents: self.opponents_move,
            yours: your_choice,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|l| l.parse::<Round>().unwrap())
        .map(|m| m.get_score())
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|l| l.parse::<StrategyGuideData>().unwrap())
        .map(|d| d.get_correct_move())
        .map(|m| m.get_score())
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
