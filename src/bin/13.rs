use std::{cmp::Ordering, str::FromStr};

use advent_of_code::helpers::ParseError;

#[derive(Clone, Debug)]
enum Element {
    List(Vec<Element>),
    Integer(u32),
}

fn compare_elements(e1: &Element, e2: &Element) -> Ordering {
    match e1 {
        Element::List(l1) => match e2 {
            Element::List(l2) => compare_lists(l1, l2),
            Element::Integer(_) => {
                let l2 = Element::List(vec![e2.clone()]);
                compare_elements(e1, &l2)
            }
        },
        Element::Integer(i1) => match e2 {
            Element::Integer(i2) => i1.cmp(i2),
            Element::List(_) => {
                let l1 = Element::List(vec![e1.clone()]);
                compare_elements(&l1, e2)
            }
        },
    }
}

fn compare_lists(l1: &Vec<Element>, l2: &Vec<Element>) -> Ordering {
    for (e1, e2) in l1.iter().zip(l2) {
        let compare = compare_elements(e1, e2);
        if compare != Ordering::Equal {
            return compare;
        };
    }

    match l1.len() as i32 - l2.len() as i32 {
        i32::MIN..=-1 => Ordering::Less,
        0 => Ordering::Equal,
        1..=i32::MAX => Ordering::Greater,
    }
}

fn parse_list_recursive(it: &mut impl Iterator<Item = char>) -> Result<Vec<Element>, ParseError> {
    let mut list = Vec::new();
    loop {
        let mut ch = it.next().ok_or(ParseError::InvalidInput)?;
        match ch {
            ',' => {}
            ']' => return Ok(list),
            '[' => list.push(Element::List(parse_list_recursive(it)?)),
            _ => {
                let mut digits = String::new();
                while ch.is_digit(10) {
                    digits.push(ch);
                    ch = it.next().ok_or(ParseError::InvalidInput)?;
                }

                list.push(Element::Integer(digits.parse::<u32>().unwrap()));
                if ch == ']' {
                    return Ok(list);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Packet {
    elements: Vec<Element>,
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        _ = it.next().ok_or(ParseError::InvalidInput)?; // first '[' char
        Ok(Self {
            elements: parse_list_recursive(&mut it)?,
        })
    }
}

#[derive(Debug)]
struct Pair {
    p1: Packet,
    p2: Packet,
}

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_it = s.lines();

        let p1 = lines_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<Packet>()?;
        let p2 = lines_it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<Packet>()?;

        Ok(Self { p1, p2 })
    }
}

impl Pair {
    pub fn is_right_order(&self) -> bool {
        for (e1, e2) in self.p1.elements.iter().zip(&self.p2.elements) {
            let compare = compare_elements(e1, e2);
            match compare {
                Ordering::Less => return true,
                Ordering::Greater => return false,
                Ordering::Equal => {}
            }
        }

        if self.p1.elements.len() == self.p2.elements.len() {
            panic!("Tie")
        }
        return self.p1.elements.len() < self.p2.elements.len();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let pairs: Vec<_> = input
        .split("\n\n")
        .map(|pair_data| pair_data.parse::<Pair>().unwrap())
        .collect();

    let mut sum = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        if pair.is_right_order() {
            sum += idx + 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
