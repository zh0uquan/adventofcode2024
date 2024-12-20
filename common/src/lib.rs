use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    matrix: Vec<Vec<T>>,
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

    /// Checks if the given row and column are within bounds.
    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.height && col < self.width
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
