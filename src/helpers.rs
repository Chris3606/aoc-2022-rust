/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::{ops, str::FromStr};

#[derive(Debug)]
pub enum ParseError {
    InvalidInput,
}

pub static CARDINAL_DIRS: [Vector2i; 4] = [
    Vector2i { x: 0, y: -1 },
    Vector2i { x: 1, y: 0 },
    Vector2i { x: 0, y: 1 },
    Vector2i { x: -1, y: 0 },
];

pub static EIGHTWAY_DIRS: [Vector2i; 8] = [
    Vector2i { x: 0, y: -1 },
    Vector2i { x: 1, y: -1 },
    Vector2i { x: 1, y: 0 },
    Vector2i { x: 1, y: 1 },
    Vector2i { x: 0, y: 1 },
    Vector2i { x: -1, y: 1 },
    Vector2i { x: -1, y: 0 },
    Vector2i { x: -1, y: -1 },
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

    /// Gets a reference to the value at the given position.
    pub fn get(&self, pos: &Vector2i) -> &T {
        &self.values[pos.to_index(self.width)]
    }

    /// Gets a mutable reference to the value at the given position.
    pub fn get_mut(&mut self, pos: &Vector2i) -> &mut T {
        &mut self.values[pos.to_index(self.width)]
    }

    /// Gets a reference to the value at the given index.
    ///
    /// The index is assumed to have been encoded by `Vector2i::to_index` (or identical math),
    /// with a width equal to the width of the grid.
    pub fn get_index(&self, index: usize) -> &T {
        &self.values[index]
    }

    pub fn get_index_mut(&mut self, index: usize) -> &mut T {
        &mut self.values[index]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.values.len() / self.width
    }

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

    pub fn contains(&self, pos: &Vector2i) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && (pos.x as usize) < self.width()
            && (pos.y as usize) < self.height()
    }
}

impl<T> Grid<T>
where
    T: std::clone::Clone,
{
    pub fn new_empty(width: usize, height: usize, default_value: T) -> Self {
        Self {
            values: vec![default_value; width * height],
            width,
        }
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
