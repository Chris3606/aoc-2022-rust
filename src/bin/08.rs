use advent_of_code::helpers::{self, Vector2i};

pub fn part_one(input: &str) -> Option<usize> {
    let treemap = helpers::grid_from_digit_grid(input);

    let mut visibility_grid = helpers::Grid::new_empty(treemap.width(), treemap.height(), false);

    // Find visible cells from the north and south ends.
    for x in 0..treemap.width() {
        let start_pos = helpers::Vector2i { x: x as i64, y: 0 };
        let mut max_val = treemap[start_pos];
        visibility_grid[start_pos] = true;
        for y in 1..treemap.height() {
            let pos = helpers::Vector2i {
                x: x as i64,
                y: y as i64,
            };

            let val = treemap[pos];
            if val > max_val {
                max_val = val;
                visibility_grid[pos] = true;
            }
        }

        let start_pos = helpers::Vector2i {
            x: x as i64,
            y: treemap.height() as i64 - 1,
        };
        let mut max_val = treemap[start_pos];
        visibility_grid[start_pos] = true;
        for y in (0..treemap.height()).rev() {
            let pos = helpers::Vector2i {
                x: x as i64,
                y: y as i64,
            };

            let val = treemap[pos];
            if val > max_val {
                max_val = val;
                visibility_grid[pos] = true;
            }
        }
    }

    // Find visible cells from the west and east ends
    for y in 0..treemap.height() {
        let start_pos = helpers::Vector2i { x: 0, y: y as i64 };
        let mut max_val = treemap[start_pos];
        visibility_grid[start_pos] = true;
        for x in 1..treemap.width() {
            let pos = helpers::Vector2i {
                x: x as i64,
                y: y as i64,
            };

            let val = treemap[pos];
            if val > max_val {
                max_val = val;
                visibility_grid[pos] = true;
            }
        }

        let start_pos = helpers::Vector2i {
            x: treemap.width() as i64 - 1,
            y: y as i64,
        };
        let mut max_val = treemap[start_pos];
        visibility_grid[start_pos] = true;
        for x in (0..treemap.width()).rev() {
            let pos = helpers::Vector2i {
                x: x as i64,
                y: y as i64,
            };

            let val = treemap[pos];
            if val > max_val {
                max_val = val;
                visibility_grid[pos] = true;
            }
        }
    }

    let visible_trees = visibility_grid.iter().filter(|&l| *l).count();

    Some(visible_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    let treemap = helpers::grid_from_digit_grid(input);

    // This is somewhat brute force, but works for our input sizes.
    let mut best_scenic_score = 0;
    for i in 0..treemap.num_cells() {
        let pos = Vector2i::new_from_index(i as u64, treemap.width() as u64);
        let val = treemap[pos];

        let mut scenic_score = 1;
        for dir in helpers::CARDINAL_DIRS {
            let mut cur = pos + &dir;
            let mut view_dist = 0;
            while treemap.contains(&cur) {
                view_dist += 1;
                if treemap[cur] >= val {
                    break;
                }
                cur = cur + &dir;
            }
            scenic_score *= view_dist;
        }
        best_scenic_score = best_scenic_score.max(scenic_score);
    }

    Some(best_scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
