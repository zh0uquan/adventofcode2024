use std::cmp::{PartialEq, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input));
}

type Coord = (usize, usize);
type Step = (Direction, Coord);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Start,
    End,
    Dot,
    Unique,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Start => write!(f, "S"),
            Tile::End => write!(f, "E"),
            Tile::Dot => write!(f, "."),
            Tile::Unique => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    matrix: Vec<Vec<Tile>>,
    start_pos: Coord,
    end_pos: Coord,
    height: usize,
    width: usize,
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.matrix {
            let row_str = v.iter().map(|n| n.to_string()).collect::<String>();
            writeln!(f, "{}", row_str)?;
        }
        Ok(())
    }
}

impl Maze {
    fn new(matrix: Vec<Vec<Tile>>) -> Self {
        let (height, width) = (matrix.len(), matrix[0].len());
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

        (0..height)
            .flat_map(|i| (0..width).map(move |j| (i, j)))
            .for_each(|(i, j)| {
                if matrix[i][j] == Tile::End {
                    end_pos = (i, j);
                }
                if matrix[i][j] == Tile::Start {
                    start_pos = (i, j);
                }
            });

        Maze {
            matrix,
            start_pos,
            end_pos,
            height,
            width,
        }
    }

    fn get_next_coords(&self, coord: Coord) -> Vec<Step> {
        [
            (Direction::Up, (coord.0.wrapping_sub(1), coord.1)),
            (Direction::Down, (coord.0 + 1, coord.1)),
            (Direction::Left, (coord.0, coord.1.wrapping_sub(1))),
            (Direction::Right, (coord.0, coord.1 + 1)),
        ]
        .into_iter()
        .filter(|&(_, (i, j))| {
            i < self.height
                && j < self.width
                && [Tile::End, Tile::Dot].contains(&self.matrix[i][j])
        })
        .collect()
    }

    fn run(&self) -> (usize, usize) {
        let step: Step = (Direction::Right, self.start_pos);
        let mut dp: HashMap<Step, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut best = HashSet::new();
        let path = vec![self.start_pos];
        let mut min_score = usize::MAX;

        heap.push(Reverse((0, step, path)));
        while let Some(Reverse((current_score, current_step, current_path))) =
            heap.pop()
        {
            if current_step.1 == self.end_pos {
                best.extend(current_path.clone());
                min_score = current_score;
            }
            dp.insert(current_step, current_score);
            for next_step in self.get_next_coords(current_step.1) {
                let next_score = if current_step.0 != next_step.0 {
                    1001 + current_score
                } else {
                    1 + current_score
                };
                if next_score < *dp.get(&next_step).unwrap_or(&usize::MAX)
                    && next_score <= min_score
                {
                    let mut next_path = current_path.clone();
                    next_path.push(next_step.1);
                    heap.push(Reverse((next_score, next_step, next_path)));
                }
            }
        }

        let mut maze = self.clone();
        for (i, j) in best.iter() {
            maze.matrix[*i][*j] = Tile::Unique;
        }
        println!("{}", maze);

        (min_score, best.len())
    }
}

fn solve(input: &str) -> (usize, usize) {
    let matrix: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Dot,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => panic!("disco"),
                })
                .collect()
        })
        .collect();

    let maze = Maze::new(matrix);
    // println!("{}", maze);

    maze.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
            "#
        };
        assert_eq!(solve(input), (7036, 45));

        let input = indoc! {
            r#"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
            "#
        };

        assert_eq!(solve(input), (11048, 64));
    }
}
