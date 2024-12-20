use common::Matrix;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Default, Debug, Clone, PartialEq)]
enum Tile {
    #[default]
    Track,
    Start,
    End,
    Wall,
    Char(char),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char_representation = match self {
            Tile::Start => "S",
            Tile::End => "E",
            Tile::Track => ".",
            Tile::Wall => "#",
            Tile::Char(char) => &*format!("{char}"),
        };
        write!(f, "{}", char_representation)
    }
}

fn parse(char: char) -> Tile {
    match char {
        'S' => Tile::Start,
        'E' => Tile::End,
        '.' => Tile::Track,
        '#' => Tile::Wall,
        _ => panic!("disco!"),
    }
}

type Coord = (usize, usize);

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

#[derive(Debug)]
struct Cheat {
    start: Coord,
    end: Coord,
    before_start: Vec<Coord>,
    after_end: Vec<Coord>,
}

trait MatrixCheat {
    fn find_cheats(&self) -> Vec<Cheat>;
    fn find_enclosed_walls(&self) -> Vec<Coord>;
    fn get_tile_neighbours(&self, pos: Coord) -> Vec<Coord>;
}

impl MatrixCheat for Matrix<Tile> {
    fn find_cheats(&self) -> Vec<Cheat> {
        let mut cheats = Vec::new();
        let enclosed_walls: Vec<Coord> = self.find_enclosed_walls();

        for (i, j) in (1..self.height - 1)
            .flat_map(|i| (1..self.width - 1).map(move |j| (i, j)))
        {
            if self.matrix[i][j] != Tile::Wall {
                continue;
            }
            let start = (i, j);
            let ends: Vec<Coord> = self
                .get_tile_neighbours(start)
                .into_iter()
                .filter(|&pos| !enclosed_walls.contains(&pos))
                .collect();
            for &end in ends.iter() {
                let after_end: Vec<Coord> = self
                    .get_tile_neighbours(end)
                    .into_iter()
                    .filter(|&pos| {
                        !enclosed_walls.contains(&pos) && pos != start
                    })
                    .collect();
                let before_start = ends
                    .clone()
                    .into_iter()
                    .filter(|&pos| pos != end)
                    .collect();
                cheats.push(Cheat {
                    start,
                    end,
                    before_start,
                    after_end,
                })
            }
        }
        cheats
    }

    fn find_enclosed_walls(&self) -> Vec<Coord> {
        (0..self.height)
            .flat_map(|i| (0..self.width).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                if i == 0
                    || i == self.height - 1
                    || j == 0
                    || j == self.width - 1
                {
                    return true;
                }
                if self.matrix[i][j] != Tile::Wall {
                    return false;
                }
                DIRECTIONS.iter().all(|&(di, dj)| {
                    self.matrix[(i as isize + di) as usize]
                        [(j as isize + dj) as usize]
                        == Tile::Wall
                })
            })
            .collect()
    }

    fn get_tile_neighbours(&self, pos: Coord) -> Vec<Coord> {
        DIRECTIONS
            .iter()
            .filter_map(|(di, dj)| {
                let new_i = pos.0 as isize + di;
                let new_j = pos.1 as isize + dj;

                (new_i >= 0 && new_j >= 0)
                    .then_some((new_i as usize, new_j as usize))
                    .filter(|&(i, j)| self.in_bounds(i, j))
            })
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let mut matrix: Matrix<Tile> = Matrix::from(input, parse);
    let start = matrix.find(&Tile::Start).unwrap();
    let end = matrix.find(&Tile::End).unwrap();

    let mut queue = BinaryHeap::new();
    let mut distance: HashMap<Coord, usize> = HashMap::new();
    let mut pathes = HashSet::new();
    queue.push(Reverse((0, start, vec![start])));
    while let Some(Reverse((curr_score, curr_pos, curr_path))) = queue.pop() {
        if *distance.get(&curr_pos).unwrap_or(&usize::MAX) <= curr_score {
            continue;
        }
        distance.insert(curr_pos, curr_score);
        if curr_pos == end {
            pathes.extend(curr_path.clone());
        }
        for next_pos in matrix.get_tile_neighbours(curr_pos) {
            if !matrix.in_bounds(next_pos.0, next_pos.1) {
                continue;
            }
            if [Tile::Track, Tile::End].contains(&matrix[next_pos])
                && *distance.get(&next_pos).unwrap_or(&usize::MAX)
                    > curr_score + 1
            {
                let mut path = curr_path.clone();
                path.push(next_pos);
                queue.push(Reverse((curr_score + 1, next_pos, path)));
            }
        }
    }

    for coord in matrix.find_enclosed_walls() {
        matrix[coord] = Tile::Char('D');
    }
    println!("{}", matrix);

    matrix.find_cheats();
    let mut counter: HashMap<usize, usize> = HashMap::new();
    for cheat in matrix.find_cheats() {
        let mut matrix = matrix.clone();
        matrix[cheat.start] = Tile::Char('1');
        matrix[cheat.end] = Tile::Char('2');
        for &before_wall in cheat.before_start.iter() {
            for &after_wall in cheat.after_end.iter() {
                if before_wall.0 != after_wall.0
                    && before_wall.1 != after_wall.1
                {
                    continue;
                }

                if let (Some(d1), Some(d2)) =
                    (distance.get(&before_wall), distance.get(&after_wall))
                {
                    if *d2 >= *d1 + 2 {
                        matrix[before_wall] = Tile::Char('B');
                        matrix[after_wall] = Tile::Char('A');
                        println!("distance saved: {}", d2 - d1 - 3);
                        println!("{}", matrix);

                        *counter.entry(d2 - d1 - 3).or_default() += 1;
                    }
                }
            }
        }
    }
    println!("{:?}", counter);
    println!("{:?}", distance);

    usize::MAX

    // println!("{:?} {:?}", score, path);
    // println!("{:?}", visited.get(&(7,4)).unwrap());
}

fn part2(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
            "#
        };
        part1(input);

        // assert_eq!();
    }

    #[test]
    fn test_part2() {
        // assert_eq!();
    }
}
