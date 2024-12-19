use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

struct Matrix<T, const HEIGHT: usize, const WIDTH: usize> {
    matrix: Vec<Vec<T>>,
}

impl<T, const HEIGHT: usize, const WIDTH: usize> Index<(usize, usize)>
    for Matrix<T, HEIGHT, WIDTH>
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.matrix[index.0][index.1]
    }
}

impl<T, const HEIGHT: usize, const WIDTH: usize> IndexMut<(usize, usize)>
    for Matrix<T, HEIGHT, WIDTH>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.matrix[index.0][index.1]
    }
}

impl<T: Clone + Default, const HEIGHT: usize, const WIDTH: usize>
    Matrix<T, HEIGHT, WIDTH>
{
    pub fn new() -> Self {
        Self {
            matrix: vec![vec![T::default(); WIDTH]; HEIGHT],
        }
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < HEIGHT && col < WIDTH
    }
}

impl<
        T: Clone + Default + Display,
        const HEIGHT: usize,
        const WIDTH: usize,
    > Display for Matrix<T, HEIGHT, WIDTH>
{
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

impl<T: Clone + Default, const HEIGHT: usize, const WIDTH: usize> Default
    for Matrix<T, HEIGHT, WIDTH>
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Memory {
    Corrupted,
    #[default]
    Space,
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Memory::Corrupted => write!(f, "#"),
            Memory::Space => write!(f, "."),
        }
    }
}

type Coord = (usize, usize);

fn parse_bytes(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (col, row) = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (row, col)
        })
        .collect()
}

fn run<const HEIGHT: usize, const WIDTH: usize>(
    matrix: &Matrix<Memory, HEIGHT, WIDTH>,
) -> usize {
    let start = (0, 0);
    let end = (HEIGHT - 1, WIDTH - 1);

    let path: Vec<Coord> = vec![start];
    let mut queue = BinaryHeap::new();
    let mut visited: HashMap<Coord, usize> = HashMap::new();
    queue.push(Reverse((0, start, path)));
    while let Some(Reverse((curr_score, curr_pos, curr_path))) = queue.pop() {
        if *visited.get(&curr_pos).unwrap_or(&usize::MAX) <= curr_score {
            continue;
        }
        visited.insert(curr_pos, curr_score);
        if curr_pos == end {
            return curr_score;
        }
        for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next_pos = (
                curr_pos.0.checked_add_signed(di),
                curr_pos.1.checked_add_signed(dj),
            );
            if next_pos.0.is_none() || next_pos.1.is_none() {
                continue;
            }
            let next_pos = (next_pos.0.unwrap(), next_pos.1.unwrap());
            if !matrix.in_bounds(next_pos.0, next_pos.1) {
                continue;
            }
            if matrix[next_pos] == Memory::Space
                && *visited.get(&next_pos).unwrap_or(&usize::MAX)
                    > curr_score + 1
            {
                let mut path = curr_path.clone();
                path.push(next_pos);
                queue.push(Reverse((curr_score + 1, next_pos, path)));
            }
        }
    }
    usize::MAX
}

fn part1(input: &str) -> usize {
    let mut matrix: Matrix<Memory, 71, 71> = Matrix::default();
    let bytes = parse_bytes(input);
    for byte in &bytes[..1024] {
        matrix[*byte] = Memory::Corrupted;
    }
    run(&matrix)
}

fn part2(input: &str) {
    let bytes = parse_bytes(input);
    for n in 1024..bytes.len() {
        println!("{:?}", n);
        let mut matrix: Matrix<Memory, 71, 71> = Matrix::default();
        for byte in &bytes[..n] {
            matrix[*byte] = Memory::Corrupted;
        }
        if run(&matrix) == usize::MAX {
            println!("{n}\n {:?}", bytes[n - 1]);
            println!("{}", matrix);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {
            r#"
            5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0
            "#
        };
        let mut matrix: Matrix<Memory, 7, 7> = Matrix::default();
        let bytes = parse_bytes(input);
        for byte in &bytes[..12] {
            matrix[*byte] = Memory::Corrupted;
        }
        assert_eq!(run(&matrix), 22);

        for n in 12..bytes.len() {
            let mut matrix: Matrix<Memory, 7, 7> = Matrix::default();
            for byte in &bytes[..n] {
                matrix[*byte] = Memory::Corrupted;
            }
            if run(&matrix) == usize::MAX {
                println!("{n}\n {:?}", bytes[n - 1]);
                println!("{}", matrix);
                break;
            }
        }
    }
}
