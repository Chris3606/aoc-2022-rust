/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::{
    fmt::Display,
    ops::{self, Index, IndexMut},
    str::FromStr,
};

#[derive(Debug)]
pub enum ParseError {
    InvalidInput,
}

pub const UP: Vector2i = Vector2i { x: 0, y: -1 };
pub const UP_RIGHT: Vector2i = Vector2i { x: 1, y: -1 };
pub const RIGHT: Vector2i = Vector2i { x: 1, y: 0 };
pub const DOWN_RIGHT: Vector2i = Vector2i { x: 1, y: 1 };
pub const DOWN: Vector2i = Vector2i { x: 0, y: 1 };
pub const DOWN_LEFT: Vector2i = Vector2i { x: -1, y: 1 };
pub const LEFT: Vector2i = Vector2i { x: -1, y: 0 };
pub const UP_LEFT: Vector2i = Vector2i { x: -1, y: -1 };

pub static CARDINAL_DIRS: [Vector2i; 4] = [UP, RIGHT, DOWN, LEFT];

pub static EIGHTWAY_DIRS: [Vector2i; 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];

pub enum AdjacencyRule {
    Cardinals,
    EightWay,
}

/// Represents a 2d Vector2i with integer coordinates.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2i {
    /// The x-value of the coordinate.  Increases to the right visually.
    pub x: i64,
    /// The y-value of the coordinate.  Increases to the bottom visually.
    pub y: i64,
}

impl FromStr for Vector2i {
    type Err = ParseError;

    /// Reads in a Vector2i from a string in format "x,y"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(",");
        let x = it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<i64>()
            .map_err(|_| ParseError::InvalidInput)?;
        let y = it
            .next()
            .ok_or(ParseError::InvalidInput)?
            .parse::<i64>()
            .map_err(|_| ParseError::InvalidInput)?;

        Ok(Vector2i { x, y })
    }
}

impl Vector2i {
    /// Creates a new Vector2i.
    pub fn new(x: i64, y: i64) -> Self {
        Vector2i { x, y }
    }

    /// Creates a new Vector2i which represents the index value given, assuming it is encoded with
    /// the given width value.
    ///
    /// Index is defined as y * width + x.
    pub fn new_from_index(index: u64, width: u64) -> Self {
        Vector2i {
            x: (index % width) as i64,
            y: (index / width) as i64,
        }
    }

    /// Gets the "index" value associated with the given position.
    ///
    /// Indexes are calculated as y * width + x.  This is useful for storing a grid of Vector2is in a
    /// 1-dimensional array.
    pub fn to_index(&self, width: usize) -> usize {
        (self.y as usize) * width + (self.x as usize)
    }

    pub fn neighbors(self, adjacency_rule: AdjacencyRule) -> NeighborIterator {
        NeighborIterator {
            point: self,
            iter: match adjacency_rule {
                AdjacencyRule::Cardinals => CARDINAL_DIRS.iter(),
                AdjacencyRule::EightWay => EIGHTWAY_DIRS.iter(),
            },
        }
    }
}

impl ops::Add<&Vector2i> for Vector2i {
    type Output = Self;

    /// Adds the corresponding x and y components of the Vector2i together.
    fn add(self, rhs: &Vector2i) -> Self {
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct NeighborIterator {
    point: Vector2i,
    iter: core::slice::Iter<'static, Vector2i>,
}

impl Iterator for NeighborIterator {
    type Item = Vector2i;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.iter.next() {
            Some(self.point + p)
        } else {
            None
        }
    }
}

pub fn chebyshev_distance(p1: &Vector2i, p2: &Vector2i) -> u64 {
    p2.x.abs_diff(p1.x).max(p2.y.abs_diff(p1.y))
}

pub fn manhattan_distance(p1: &Vector2i, p2: &Vector2i) -> u64 {
    p2.x.abs_diff(p1.x) + p2.y.abs_diff(p1.y)
}
/// Represents a 2-dimensional, integral grid of values.
///
/// It stores the backing values in a vector of the appropriate size, and allows you to access
/// and/or iterate over them both by position and index.
#[derive(Debug)]
pub struct Grid<T> {
    /// The values in the grid.
    values: Vec<T>,
    /// The width of the grid.
    width: usize,
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T> Index<Vector2i> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Vector2i) -> &Self::Output {
        &self.values[pos.to_index(self.width)]
    }
}

impl<T> IndexMut<Vector2i> for Grid<T> {
    fn index_mut(&mut self, pos: Vector2i) -> &mut Self::Output {
        &mut self.values[pos.to_index(self.width)]
    }
}

impl<T> Grid<T> {
    /// Creates a new grid based on the values in the given vector, and the width specified.
    ///
    /// The values vector should have indices which corresponds to positions as defined in the
    /// `Vector2i::to_index` function.  The vector must have exactly width * height values (and in fact
    /// the height of the grid is inferred from the grid size and the width specified).
    pub fn new(values: Vec<T>, width: usize) -> Self {
        assert!(values.len() % width == 0);
        Self { values, width }
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.values.len() / self.width
    }

    /// Returns the total number of cells in the grid (will always be width() * height())
    pub fn num_cells(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.values.into_iter()
    }

    /// Returns whether or not the given position is contained within this grid.
    pub fn contains(&self, pos: &Vector2i) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && (pos.x as usize) < self.width()
            && (pos.y as usize) < self.height()
    }

    pub fn positions(&self) -> impl Iterator<Item = Vector2i> + '_ {
        (0..self.values.len()).map(|i| Vector2i::new_from_index(i as u64, self.width() as u64))
    }
}

impl<T> Grid<T>
where
    T: std::clone::Clone,
{
    /// Creates a new grid of the given size, with every cell initialized to the given default value.
    pub fn new_empty(width: usize, height: usize, default_value: T) -> Self {
        Self {
            values: vec![default_value; width * height],
            width,
        }
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Vector2i {
                    x: x as i64,
                    y: y as i64,
                };
                write!(f, "{}", self[pos])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

/// Parses a Grid<u32> from a string of the following format:
/// ```
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// ```
/// The grid can contain only the digits 0-9.
pub fn grid_from_digit_grid(input: &str) -> Grid<u32> {
    let values: Vec<_> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    let width = input.find('\n').unwrap();
    Grid::new(values, width)
}

/// Calculate the least common multiple of the given numbers.
pub fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

/// Calculate the greatest common divisor of the given numbers.
pub fn gcd(first: u64, second: u64) -> u64 {
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
