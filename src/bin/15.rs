use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

use advent_of_code::helpers::{manhattan_distance, ParseError, Vector2i};

#[derive(Debug)]
struct Sensor {
    position: Vector2i,
    dist_to_closest_beacon: u64,
}

#[derive(Debug)]
struct MapData {
    /// Sensors
    sensors: Vec<Sensor>,
    /// Positions of beacons detected by sensors
    beacons: HashSet<Vector2i>,
}

impl FromStr for MapData {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Self {
            sensors: Vec::new(),
            beacons: HashSet::new(),
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

            data.sensors.push(Sensor {
                position: sensor,
                dist_to_closest_beacon: manhattan_distance(&sensor, &beacon),
            });
        }

        Ok(data)
    }
}

fn ranges_overlap(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    r1.end() + 1 == *r2.start()
        || r2.end() + 1 == *r1.start()
        || r2.contains(r1.start())
        || r2.contains(r1.end())
        || r1.contains(r2.start())
        || r1.contains(r2.end())
}

fn join_ranges(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    RangeInclusive::new(*r1.start().min(r2.start()), *r1.end().max(r2.end()))
}

struct RangeSet {
    ranges: Vec<RangeInclusive<i64>>,
}

impl RangeSet {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn insert(&mut self, range: RangeInclusive<i64>) {
        let mut overlapping_range = None;

        for (idx, r) in self.ranges.iter().enumerate() {
            if ranges_overlap(&range, r) {
                overlapping_range = Some(idx);
                break;
            }
        }

        if let Some(idx) = overlapping_range {
            let overlapping = self.ranges.remove(idx);
            self.insert(join_ranges(&overlapping, &range));
        } else {
            self.ranges.push(range);
        }
    }

    pub fn nums_included(&self) -> i64 {
        self.ranges.iter().map(|r| *r.end() - *r.start() + 1).sum()
    }
}

/// Given a row, calculates the positions at which a beacon _cannot_ be, and places the x-value of those positions
/// in the RangeSet.
fn get_impossible_positions_for_row(set: &mut RangeSet, map: &MapData, row: i64) {
    set.ranges.clear();

    for sensor in &map.sensors {
        let y_dist = sensor.position.y.abs_diff(row);

        // Sensor can't overlap with search region
        if y_dist > sensor.dist_to_closest_beacon {
            continue;
        }

        let x_dist = sensor.dist_to_closest_beacon as i64 - y_dist as i64;

        // It does overlap, starting at the center and as far out as manhattan distance allows
        let x_min = sensor.position.x - x_dist;
        let x_max = sensor.position.x + x_dist;

        set.insert(x_min..=x_max);
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    // Use this y value for solving
    const Y_VAL: i64 = 2000000;
    // Use this y value for tests
    //const Y_VAL: i64 = 10;

    let data = input.parse::<MapData>().unwrap();

    let mut impossible_positions = RangeSet::new();
    get_impossible_positions_for_row(&mut impossible_positions, &data, Y_VAL);

    Some(
        impossible_positions.nums_included()
            - data.beacons.iter().filter(|&i| i.y == Y_VAL).count() as i64,
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    // Use this max value for solving
    const MAX_COORD: i64 = 4000000;
    // Use this max value for tests
    //const MAX_COORD: i64 = 20;

    let data = input.parse::<MapData>().unwrap();

    for y in 0..=MAX_COORD {
        let mut impossible_positions = RangeSet::new();
        get_impossible_positions_for_row(&mut impossible_positions, &data, y);

        //Find a gap in the impossible values within the coordinate range; this by definition is a possible position
        let mut x = 0;
        'x_loop: while x <= MAX_COORD {
            for range in &impossible_positions.ranges {
                if range.contains(&x) {
                    x = *range.end() + 1;
                    continue 'x_loop;
                }
            }

            break;
        }

        if x <= MAX_COORD {
            return Some(x * 4000000 + y);
        }
    }

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
        assert_eq!(part_two(&input), Some(56000011));
    }
}
