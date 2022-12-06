use std::collections::HashSet;

fn find_marker(sequence: &Vec<char>, seq_len: usize) -> Option<usize> {
    for i in 0..sequence.len() - seq_len {
        let map: HashSet<char> = sequence[i..i + seq_len].iter().map(|i| *i).collect();
        if map.len() == seq_len {
            return Some(i + seq_len);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    find_marker(&chars, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    find_marker(&chars, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
