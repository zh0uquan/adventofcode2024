use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub type Coord = (usize, usize);
pub type Direction = (isize, isize);

pub const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone)]
pub struct Matrix<T> {
    pub matrix: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.matrix[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.matrix[index.0][index.1]
    }
}

impl<T: Clone + Default> Matrix<T> {
    /// Creates a new matrix with the given height and width, filled with default values.
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            matrix: vec![vec![T::default(); width]; height],
            height,
            width,
        }
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width
    }

    pub fn in_bounds_isize(&self, row: isize, col: isize) -> bool {
        row >= 0
            && col >= 0
            && (row as usize) < self.height
            && (col as usize) < self.width
    }

    pub fn get_coord_neighbours(&self, pos: Coord) -> Vec<(Coord, Direction)> {
        DIRECTIONS
            .iter()
            .filter_map(|&(di, dj)| {
                let (ni, nj) = (pos.0 as isize + di, pos.1 as isize + dj);
                if self.in_bounds_isize(ni, nj) {
                    return Some(((ni as usize, nj as usize), (di, dj)));
                }
                None
            })
            .collect()
    }

    pub fn find(&self, value: &T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell == value {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn from<F>(input: &str, parser_func: F) -> Self
    where
        F: Fn(char) -> T + Copy,
    {
        let matrix: Vec<Vec<T>> = input
            .lines()
            .map(|line| line.chars().map(parser_func).collect())
            .collect();

        let height = matrix.len();
        let width = matrix[0].len();

        Self {
            matrix,
            height,
            width,
        }
    }
}

impl<T: Clone + Default + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
