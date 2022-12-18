use std::collections::HashSet;

use advent_of_code::helpers::{Vector2i, DOWN, DOWN_LEFT, DOWN_RIGHT};

fn parse_input(input: &str) -> HashSet<Vector2i> {
    let mut map = HashSet::new();

    for line in input.lines() {
        let points: Vec<_> = line
            .split(" -> ")
            .map(|p| p.parse::<Vector2i>().unwrap())
            .collect();

        for i in 0..points.len() - 1 {
            let start = points[i];
            let end = points[i + 1];

            if start.x != end.x {
                let min = start.x.min(end.x);
                let max = start.x.max(end.x);
                for x in min..=max {
                    map.insert(Vector2i::new(x, start.y));
                }
            } else {
                let min = start.y.min(end.y);
                let max = start.y.max(end.y);
                for y in min..=max {
                    map.insert(Vector2i::new(start.x, y));
                }
            }
        }
    }

    map
}

fn simulate_grain_of_sand(
    map: &HashSet<Vector2i>,
    max_y: i64,
    entry_point: Vector2i,
) -> Option<Vector2i> {
    let mut cur = entry_point;
    loop {
        // Sand will fall into infinity since there's nothing left below it to block it
        if cur.y > max_y {
            break;
        }

        // Check neighbors in order
        let n = cur + &DOWN;
        if !map.contains(&n) {
            cur = n;
            continue;
        }

        let n = cur + &DOWN_LEFT;
        if !map.contains(&n) {
            cur = n;
            continue;
        }

        let n = cur + &DOWN_RIGHT;
        if !map.contains(&n) {
            cur = n;
            continue;
        }

        // Can't go anywhere; found final point for sand grain
        return Some(cur);
    }

    None
}

fn simulate_grain_of_sand_2(
    map: &HashSet<Vector2i>,
    max_y: i64,
    entry_point: Vector2i,
) -> Vector2i {
    let mut cur = entry_point;
    loop {
        // Sand has hit the floor
        if cur.y == max_y + 2 - 1 {
            return cur;
        }

        // Check neighbors in order
        let n = cur + &DOWN;
        if !map.contains(&n) {
            cur = n;
            continue;
        }

        let n = cur + &DOWN_LEFT;
        if !map.contains(&n) {
            cur = n;
            continue;
        }

        let n = cur + &DOWN_RIGHT;
        if !map.contains(&n) {
            cur = n;
            continue;
        }

        // Can't go anywhere; found final point for sand grain
        return cur;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_input(input);
    let max_y = map.iter().map(|p| p.y).max().unwrap();
    let entry_point = Vector2i::new(500, 0);

    let mut sand_grains = 0;
    loop {
        if let Some(sand) = simulate_grain_of_sand(&map, max_y, entry_point) {
            map.insert(sand);
            sand_grains += 1;
        } else {
            break;
        }
    }

    Some(sand_grains)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_input(input);
    let max_y = map.iter().map(|p| p.y).max().unwrap();
    let entry_point = Vector2i::new(500, 0);

    let mut sand_grains = 0;
    loop {
        let sand = simulate_grain_of_sand_2(&map, max_y, entry_point);
        map.insert(sand);
        sand_grains += 1;

        if sand == entry_point {
            return Some(sand_grains);
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
