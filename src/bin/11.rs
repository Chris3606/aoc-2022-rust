use std::str::FromStr;

use advent_of_code::helpers::ParseError;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operation {
    AddConst(u64),
    AddOld,
    MultConst(u64),
    MultOld,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    current_items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    true_dest: usize,
    false_dest: usize,
    items_inspected: u64,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_it = s.lines();

        let mut id_data = &lines_it.next().ok_or(ParseError::InvalidInput)?["Monkey ".len()..];
        id_data = &id_data[..id_data.len() - 1];
        let id: usize = id_data.parse().map_err(|_| ParseError::InvalidInput)?;

        let current_items_data =
            &lines_it.next().ok_or(ParseError::InvalidInput)?["  Starting items: ".len()..];
        let current_items: Vec<u64> = current_items_data
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let mut operation_it = (&lines_it.next().ok_or(ParseError::InvalidInput)?
            ["  Operation: new = ".len()..])
            .split(" ");
        _ = operation_it.next().ok_or(ParseError::InvalidInput)?; // old
        let op = operation_it.next().ok_or(ParseError::InvalidInput)?;
        let operand = operation_it.next().ok_or(ParseError::InvalidInput)?;
        let operation = match op {
            "+" => match operand {
                "old" => Operation::AddOld,
                s => Operation::AddConst(s.parse().unwrap()),
            },
            "*" => match operand {
                "old" => Operation::MultOld,
                s => Operation::MultConst(s.parse().unwrap()),
            },
            _ => return Err(ParseError::InvalidInput),
        };

        let test_divisor: u64 = (&lines_it.next().ok_or(ParseError::InvalidInput)?
            ["  Test: divisible by ".len()..])
            .parse()
            .map_err(|_| ParseError::InvalidInput)?;

        let true_dest: usize = (&lines_it.next().ok_or(ParseError::InvalidInput)?
            ["    If true: throw to monkey ".len()..])
            .parse()
            .map_err(|_| ParseError::InvalidInput)?;

        let false_dest: usize = (&lines_it.next().ok_or(ParseError::InvalidInput)?
            ["    If false: throw to monkey ".len()..])
            .parse()
            .map_err(|_| ParseError::InvalidInput)?;

        Ok(Self {
            id,
            current_items,
            operation,
            test_divisor,
            true_dest,
            false_dest,
            items_inspected: 0,
        })
    }
}

fn simulate_monkey_business(monkeys: &mut Vec<Monkey>, rounds: u32) {
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            assert!(monkeys[monkey_idx].id == monkey_idx);

            for item_idx in 0..monkeys[monkey_idx].current_items.len() {
                // Record that this monkey inspected an item
                monkeys[monkey_idx].items_inspected += 1;

                let mut item = monkeys[monkey_idx].current_items[item_idx];

                // Incrase worry level
                item = match monkeys[monkey_idx].operation {
                    Operation::AddConst(c) => item + c,
                    Operation::AddOld => item + item,
                    Operation::MultConst(c) => item * c,
                    Operation::MultOld => item * item,
                };

                // Bored with item; divide by 3
                item /= 3;

                // Figure out who we throw the item to
                let throw_to = if item % monkeys[monkey_idx].test_divisor == 0 {
                    monkeys[monkey_idx].true_dest
                } else {
                    monkeys[monkey_idx].false_dest
                };

                // Throw to the proper monkey
                monkeys[throw_to].current_items.push(item);
            }
            monkeys[monkey_idx].current_items.clear();
        }
    }
}

fn simulate_monkey_business_infinite_worry(monkeys: &mut Vec<Monkey>, rounds: u32) {
    // Calculate the least common multiple of all the test divisors so we can avoid infinite growth.
    let test_divisors: Vec<u64> = monkeys.iter().map(|m| m.test_divisor).collect();
    let mut l = test_divisors[0];
    for &num in test_divisors.iter().skip(1) {
        l = lcm(l, num);
    }

    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            assert!(monkeys[monkey_idx].id == monkey_idx);

            for item_idx in 0..monkeys[monkey_idx].current_items.len() {
                // Record that this monkey inspected an item
                monkeys[monkey_idx].items_inspected += 1;

                let mut item = monkeys[monkey_idx].current_items[item_idx];

                // Incrase worry level
                item = match monkeys[monkey_idx].operation {
                    Operation::AddConst(c) => item + c,
                    Operation::AddOld => item + item,
                    Operation::MultConst(c) => item * c,
                    Operation::MultOld => item * item,
                };

                // We only use the worry level to divide by the test divisors; so mathematically
                // it's safe to cut it down to the lcm of all the test_divisors
                item %= l;

                // Figure out who we throw the item to
                let throw_to = if item % monkeys[monkey_idx].test_divisor == 0 {
                    monkeys[monkey_idx].true_dest
                } else {
                    monkeys[monkey_idx].false_dest
                };

                // Throw to the proper monkey
                monkeys[throw_to].current_items.push(item);
            }
            monkeys[monkey_idx].current_items.clear();
        }
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|m_data| m_data.parse().unwrap())
        .collect();

    simulate_monkey_business(&mut monkeys, 20);

    let mut items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
    items_inspected.sort();
    Some(items_inspected[items_inspected.len() - 2] * items_inspected[items_inspected.len() - 1])
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|m_data| m_data.parse().unwrap())
        .collect();

    simulate_monkey_business_infinite_worry(&mut monkeys, 10000);

    let mut items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
    items_inspected.sort();
    Some(items_inspected[items_inspected.len() - 2] * items_inspected[items_inspected.len() - 1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
