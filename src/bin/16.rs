use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use advent_of_code::helpers::ParseError;

#[derive(Debug)]
struct Node {
    id: String,
    flow_rate: u32,
    neighbors: Vec<String>,
}

// impl FromStr for Node {
//     type Err = ParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let node_data = s;
//         //let adjacency_data = parts_it.next().ok_or(ParseError::InvalidInput)?;

//         let node_data = &node_data["Valve ".len()..];
//         let id = &node_data[0..2];

//         let node_data = &node_data[" has flow rate=".len() + 2..];
//         let flow_rate = node_data
//             .parse::<u32>()
//             .map_err(|_| ParseError::InvalidInput)?;

//         let node = Node {
//             id: id.to_string(),
//             flow_rate,
//             neighbors: Vec::new(),
//         };

//         Ok(node)
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    cost: usize,
    position: &'a str,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn get_node_idx(&self, id: &str) -> Option<usize> {
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.id == id {
                return Some(idx);
            }
        }

        None
    }

    pub fn get_idx_or_new(&mut self, id: &str) -> usize {
        if let Some(idx) = self.get_node_idx(id) {
            return idx;
        }

        self.nodes.push(Node {
            id: id.to_string(),
            flow_rate: 0,
            neighbors: Vec::new(),
        });

        self.nodes.len() - 1
    }

    pub fn shortest_path_len(&self, from: &str, to: &str) -> Option<usize> {
        // dist[node] = current shortest distance from `start` to `node`
        let mut dist: HashMap<&str, usize> = (0..self.nodes.len())
            .map(|s| (&self.nodes[s].id[..], usize::MAX))
            .collect();

        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost
        dist.insert(from, 0);
        heap.push(State {
            cost: 0,
            position: from,
        });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == to {
                return Some(cost);
            }

            // Important as we may have already found a better way
            if cost > dist[position] {
                continue;
            }

            let node_idx = self.get_node_idx(position).unwrap();

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in &self.nodes[node_idx].neighbors {
                let next = State {
                    cost: cost + 1,
                    position: &edge[..],
                };

                // If so, add it to the frontier and continue
                if next.cost < dist[next.position] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist.insert(next.position, next.cost);
                }
            }
        }

        // Goal not reachable
        None
    }
}

impl FromStr for Graph {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();

        for line in s.lines() {
            let mut parts_it = line.split("; ");
            let node_data = parts_it.next().ok_or(ParseError::InvalidInput)?;
            let adjacency_data = parts_it.next().ok_or(ParseError::InvalidInput)?;

            let node_data = &node_data["Valve ".len()..];
            let id = &node_data[0..2];

            let node_data = &node_data[" has flow rate=".len() + 2..];
            let flow_rate = node_data
                .parse::<u32>()
                .map_err(|_| ParseError::InvalidInput)?;

            let node_idx = graph.get_idx_or_new(id);
            graph.nodes[node_idx].flow_rate = flow_rate;

            let mut adj_it = adjacency_data.split(", ");
            let first = adj_it.next().ok_or(ParseError::InvalidInput)?;
            let first = &first[first.len() - 2..];
            graph.nodes[node_idx].neighbors.push(first.to_string());

            while let Some(str) = adj_it.next() {
                graph.nodes[node_idx].neighbors.push(str.to_string());
            }
        }
        Ok(graph)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = input.parse::<Graph>().unwrap();

    // let flow_zero = graph
    //     .nodes
    //     .iter()
    //     .map(|n| n.flow_rate)
    //     .filter(|&i| i == 0)
    //     .count();

    let mut flow_zero: HashSet<usize> = graph
        .nodes
        .iter()
        .enumerate()
        //.map(|n| n.flow_rate)
        .filter(|&(idx, n)| n.flow_rate == 0)
        .map(|(idx, _)| idx)
        .collect();

    let mut cur_pos = "AA";
    let mut pressure_relieved = 0;
    let mut minutes_remaining = 30;
    while minutes_remaining != 0 && flow_zero.len() != graph.nodes.len() {
        // Find the node that, when we get to it, can relieve the most amount of pressure using the remaining time
        let mut max_pressure_relief = 0;
        let mut best_node = "";
        let mut time_to_open_best = 0;
        for (_, node) in graph
            .nodes
            .iter()
            .enumerate()
            .filter(|(idx, _)| !flow_zero.contains(idx))
        {
            // Can relieve pressure equal to flow rate, times the number of minutes remaining after the valve is opened.
            // We need to subtract the time it took to get there, and the 1 minute we took to open the valve once we get there.
            let time_to_open = graph.shortest_path_len(cur_pos, &node.id[..]).unwrap() + 1;
            let pressure_relieved = (minutes_remaining - time_to_open) * node.flow_rate as usize;

            if pressure_relieved > max_pressure_relief {
                max_pressure_relief = pressure_relieved;
                best_node = &node.id[..];
                time_to_open_best = time_to_open;
            }
        }

        // Go there and open that valve.  Also ensure we don't open it in the future.
        cur_pos = best_node;
        minutes_remaining -= time_to_open_best;
        pressure_relieved += max_pressure_relief;

        println!(
            "Opening {} by min {} to relieve {} pressure over remaining time",
            best_node,
            30 - minutes_remaining,
            max_pressure_relief
        );

        let idx = graph.get_node_idx(best_node).unwrap();
        flow_zero.insert(idx);
    }

    Some(pressure_relieved)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
