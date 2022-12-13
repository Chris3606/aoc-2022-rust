use advent_of_code::helpers::{manhattan_distance, AdjacencyRule, Grid, Vector2i};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

// Priority queue element
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    f: u32,
    g: u32,
    position: usize,
}

// Ord required for binary heap; ensure we implement such that we get _minimum_ cost
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ordering flipped, we want a _min_ heap
        // we compare positions on ties, just so that heap PartialOrd is consistent with this.
        other
            .f
            .cmp(&self.f)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Given a grid of costs, and a start and end point, finds the shortest path between them using
// A*, and returns the sum of the costs along that route.
fn shortest_path(grid: &Grid<u8>, start: &Vector2i, end: &Vector2i) -> Option<u32> {
    let mut dist = Grid::new(grid.iter().map(|_| u32::MAX).collect(), grid.width());
    let mut heap = BinaryHeap::new();

    dist[*start] = 0;
    heap.push(Node {
        g: 0,
        f: manhattan_distance(start, end) as u32,
        position: start.to_index(grid.width()),
    });

    while let Some(Node { f: _, g, position }) = heap.pop() {
        let position = Vector2i::new_from_index(position as u64, grid.width() as u64);

        // We found the shortest path
        if position == *end {
            return Some(g);
        }

        // IF we've already found a better way, we won't visit this node on the current path;
        // this can happen if multiple states with the same value were pushed into the queue
        if g > dist[position] {
            continue;
        }

        // Test all neighbors to see if there is a better path to them by going though the current
        // position
        for neighbor in position
            .neighbors(AdjacencyRule::Cardinals)
            .filter(|i| grid.contains(i))
            .filter(|i| grid[*i] <= grid[position] + 1)
        {
            let next_state = Node {
                g: g + 1,
                f: g + 1 + manhattan_distance(&neighbor, end) as u32,
                position: neighbor.to_index(grid.width()),
            };

            // If cost is lower, add it to the list of nodes to visit and update the cost
            if next_state.g < dist[neighbor] {
                dist[neighbor] = next_state.g;
                heap.push(next_state);
            }
        }
    }

    None
}

fn parse_input(input: &str) -> (Grid<u8>, Vector2i, Vector2i) {
    let mut start: Option<Vector2i> = None;
    let mut end: Option<Vector2i> = None;

    let mut values = Vec::new();
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        width = line.len();
        for (x, ch) in line.chars().enumerate() {
            let pos = Vector2i::new(x as i64, y as i64);
            let ch = match ch {
                'S' => {
                    start = Some(pos);
                    'a'
                }
                'E' => {
                    end = Some(pos);
                    'z'
                }
                _ => ch,
            };

            values.push(ch as u8);
        }
    }

    (Grid::new(values, width), start.unwrap(), end.unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse_input(input);

    let shortest_path_len = shortest_path(&grid, &start, &end);
    shortest_path_len
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _, end) = parse_input(input);

    let shortest_path_len = grid
        .positions()
        .filter(|&i| grid[i] == 'a' as u8)
        .filter_map(|i| shortest_path(&grid, &i, &end))
        .min();

    shortest_path_len
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
