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
    #[allow(dead_code)]
    Char(char),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char_representation = match self {
            Tile::Start => "S",
            Tile::End => "E",
            Tile::Track => ".",
            Tile::Wall => "#",
            Tile::Char(char) => return write!(f, "{}", char),
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
    println!("{:?}", solve(input));
}

trait MatrixCheat {
    fn find_next(&self, pos: Coord, dir: (isize, isize)) -> Option<Coord>;
    fn get_tile_neighbours(&self, pos: Coord) -> Vec<Coord>;
}

impl MatrixCheat for Matrix<Tile> {
    fn find_next(
        &self,
        pos: Coord,
        dir: (isize, isize),
    ) -> Option<(usize, usize)> {
        let ni = pos.0.checked_add_signed(dir.0);
        let nj = pos.1.checked_add_signed(dir.1);
        if let (Some(ni), Some(nj)) = (ni, nj) {
            if !self.in_bounds(ni, nj) {
                return None;
            }
            return Some((ni, nj));
        }
        None
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
fn manhattan_distance(pos1: Coord, pos2: Coord) -> usize {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}

fn solve(input: &str) -> (usize, usize) {
    let matrix: Matrix<Tile> = Matrix::from(input, parse);
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

    let mut cheats = vec![];
    for (&coord, &coord_distance) in distance.iter() {
        for &dir in DIRECTIONS.iter() {
            if let Some(start) = matrix.find_next(coord, dir) {
                if matrix[start] == Tile::Wall {
                    if let Some(end) = matrix.find_next(start, dir) {
                        // coord => start => end => out
                        if matrix[end] == Tile::Wall {
                            if let Some(out) = matrix.find_next(end, dir) {
                                if let Some(&d_out) = distance.get(&out) {
                                    cheats.push((
                                        start,
                                        end,
                                        d_out
                                            - coord_distance
                                            - manhattan_distance(coord, out),
                                    ));
                                    continue;
                                }
                            }
                        }
                        // coord => start => end
                        if let Some(&d_end) = distance.get(&end) {
                            if d_end < coord_distance {
                                continue;
                            }
                            cheats.push((
                                start,
                                end,
                                d_end
                                    - coord_distance
                                    - manhattan_distance(coord, end),
                            ));
                            continue;
                        }
                    }
                }
            }
        }
    }

    // for cheat in cheats.iter() {
    //     let mut matrix = matrix.clone();
    //     matrix[cheat.0] = Tile::Char('1');
    //     matrix[cheat.1] = Tile::Char('2');
    //     println!("distance saved: {}", cheat.2);
    //     println!("{}", matrix);
    // }
    let counter: HashMap<usize, usize> =
        cheats.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(c.2).or_default() += 1;
            map
        });

    let calculate_100 = |counter: HashMap<usize, usize>| {
        counter
            .iter()
            .filter_map(|(&pico, &n)| {
                if pico >= 100 {
                    return Some(n);
                }
                None
            })
            .sum()
    };

    let part1 = calculate_100(counter);

    let mut cheats: HashMap<usize, usize> = HashMap::new();
    for (&pos1, &distance1) in distance.iter() {
        for (&pos2, &distance2) in distance.iter() {
            if pos1 == pos2 {
                continue;
            }
            let pico = manhattan_distance(pos1, pos2);
            if pico <= 20 && distance2 > distance1 + pico {
                *cheats.entry(distance2 - distance1 - pico).or_default() += 1
            }
        }
    }

    // println!("{:?}", cheats);
    for (key, value) in cheats.iter().sorted() {
        if *key < 50 {
            continue;
        }
        println!("There are {value} cheats that save {key} picoseconds.")
    }
    let part2 = calculate_100(cheats);

    (part1, part2)
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
        solve(input);
    }
}
