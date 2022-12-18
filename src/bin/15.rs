use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use advent_of_code::helpers::{manhattan_distance, ParseError, Vector2i};

#[derive(Debug)]
struct MapData {
    /// Beacons detected
    beacons: HashSet<Vector2i>,
    /// Mapping of each sensor position to the manhattan distance between it and the beacon
    /// it detects.
    sensor_dists: HashMap<Vector2i, u64>,
}

impl FromStr for MapData {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Self {
            beacons: HashSet::new(),
            sensor_dists: HashMap::new(),
        };

        for line in s.lines() {
            let mut parts_it = line.split(": ");
            let sensor_data = parts_it.next().ok_or(ParseError::InvalidInput)?;
            let beacon_data = parts_it.next().ok_or(ParseError::InvalidInput)?;

            let mut xy_it = sensor_data.split(", ");
            let x = (&(xy_it.next().ok_or(ParseError::InvalidInput)?))["Sensor at x=".len()..]
                .parse::<i64>()
                .map_err(|_| ParseError::InvalidInput)?;
            let y = (&(xy_it.next().ok_or(ParseError::InvalidInput)?))["y=".len()..]
                .parse::<i64>()
                .map_err(|_| ParseError::InvalidInput)?;

            let sensor = Vector2i { x, y };

            let mut xy_it = beacon_data.split(", ");
            let x = (&(xy_it.next().ok_or(ParseError::InvalidInput)?))
                ["closest beacon is at x=".len()..]
                .parse::<i64>()
                .map_err(|_| ParseError::InvalidInput)?;
            let y = (&(xy_it.next().ok_or(ParseError::InvalidInput)?))["y=".len()..]
                .parse::<i64>()
                .map_err(|_| ParseError::InvalidInput)?;

            let beacon = Vector2i { x, y };
            data.beacons.insert(beacon);

            data.sensor_dists
                .insert(sensor, manhattan_distance(&sensor, &beacon));
        }

        Ok(data)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const Y_VAL: i64 = 2000000;

    let data = input.parse::<MapData>().unwrap();

    let x_min = data
        .sensor_dists
        .iter()
        .map(|(pos, &dist)| pos.x - dist as i64)
        .min()
        .unwrap();

    let x_max = data
        .sensor_dists
        .iter()
        .map(|(pos, &dist)| pos.x + dist as i64)
        .max()
        .unwrap();

    let mut num_pos = 0;
    for x in x_min..=x_max {
        let pos = Vector2i { x, y: Y_VAL };
        if data.beacons.contains(&pos) {
            continue;
        }

        for (sensor, &dist) in &data.sensor_dists {
            if manhattan_distance(sensor, &pos) <= dist {
                num_pos += 1;
                break;
            }
        }
    }

    Some(num_pos)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
