use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_direction(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

struct Game {
    matrix: Vec<Vec<char>>,
    m: isize,
    n: isize,
    start: (isize, isize),
    directions: Vec<Direction>,
}

impl Game {
    fn new(input: &str) -> Self {
        let (matrix_str, directions_str): (&str, &str) =
            input.split("\n\n").collect_tuple().unwrap();
        let matrix: Vec<Vec<char>> = matrix_str
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let directions: Vec<Direction> = directions_str
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '>' => Direction::Right,
                '<' => Direction::Left,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => panic!("disco!"),
            })
            .collect();
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut pos_i, mut pos_j) = (0, 0);
        for (i, j) in (0..m).flat_map(|i| (0..n).map(move |j| (i, j))) {
            if matrix[i][j] == '@' {
                (pos_i, pos_j) = (i as isize, j as isize);
            }
        }

        Game {
            matrix,
            m: m as isize,
            n: n as isize,
            directions,
            start: (pos_i, pos_j),
        }
    }

    fn get_next_empty(
        &self,
        dir: &Direction,
        pos_i: isize,
        pos_j: isize,
    ) -> Option<(isize, isize)> {
        let line: Vec<(isize, isize)> = match &dir {
            Direction::Up => {
                (0..pos_i - 1).map(|i| (i, pos_j)).rev().collect()
            }
            Direction::Down => {
                (pos_i + 1..self.m).map(|i| (i, pos_j)).collect()
            }
            Direction::Left => {
                (0..pos_j - 1).map(|j| (pos_i, j)).rev().collect()
            }
            Direction::Right => {
                (pos_j + 1..self.n).map(|j| (pos_i, j)).collect()
            }
        };
        for (i, j) in line {
            if i >= 1 && i < self.m - 1 && j >= 1 && j < self.n - 1 {
                if self.matrix[i as usize][j as usize] == '#' {
                    return None;
                }
                if self.matrix[i as usize][j as usize] == '.' {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn run(&mut self) -> isize {
        let (mut pos_i, mut pos_j) = self.start;
        for dir in self.directions.iter() {
            let (di, dj) = dir.get_direction();
            // println!("{:?}", dir);

            if pos_i + di >= 0
                && pos_i + di < self.m
                && pos_j + dj >= 0
                && pos_j + dj < self.n
            {
                let (next_i, next_j) = (pos_i + di, pos_j + dj);
                match self.matrix[next_i as usize][next_j as usize] {
                    '#' => continue,
                    '.' => {
                        self.matrix[pos_i as usize][pos_j as usize] = '.';
                        self.matrix[next_i as usize][next_j as usize] = '@';
                        (pos_i, pos_j) = (next_i, next_j);
                    }
                    'O' => {
                        if let Some((empty_i, empty_j)) =
                            self.get_next_empty(dir, pos_i, pos_j)
                        {
                            self.matrix[pos_i as usize][pos_j as usize] = '.';
                            self.matrix[next_i as usize][next_j as usize] =
                                '@';
                            self.matrix[empty_i as usize][empty_j as usize] =
                                'O';
                            (pos_i, pos_j) = (next_i, next_j);
                        }
                    }
                    _ => panic!("disco!"),
                }
            }
            // for v in self.matrix.iter() {
            //     println!("{}", v.iter().map(|n| n.to_string()).collect::<String>());
            // }
            // println!();
        }

        (0..self.m)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .map(|(i, j)| {
                if self.matrix[i as usize][j as usize] == 'O' {
                    return 100 * i + j;
                }
                0
            })
            .sum()
    }

    fn expand_matrix(&mut self) {
        let matrix: Vec<Vec<char>> = (0..self.m)
            .map(|i| {
                (0..self.n)
                    .flat_map(|j| match self.matrix[i as usize][j as usize] {
                        '#' => vec!['#', '#'],
                        'O' => vec!['[', ']'],
                        '.' => vec!['.', '.'],
                        '@' => vec!['@', '.'],
                        _ => panic!("disco!"),
                    })
                    .collect()
            })
            .collect();

        for v in matrix.iter() {
            println!(
                "{}",
                v.iter().map(|n| n.to_string()).collect::<String>()
            );
        }

        self.m = matrix.len() as isize;
        self.n = matrix[0].len() as isize;
        for (i, j) in
            (0..self.m).flat_map(|i| (0..self.n).map(move |j| (i, j)))
        {
            if matrix[i as usize][j as usize] == '@' {
                self.start = (i, j);
            }
        }
        self.matrix = matrix;
    }

    fn is_in_bound(&self, i: isize, j: isize) -> bool {
        if i >= 0 && i < self.m && j >= 0 && j < self.n {
            return true;
        }
        false
    }

    fn get_up_or_down_grid(
        &self,
        pos_i: isize,
        pos_j: isize,
        di: isize,
    ) -> Option<HashSet<(isize, isize)>> {
        let mut level = if self.matrix[pos_i as usize][pos_j as usize] == '[' {
            vec![(pos_i, pos_j), (pos_i, pos_j + 1)]
        } else {
            vec![(pos_i, pos_j - 1), (pos_i, pos_j)]
        };
        let mut finals = HashSet::new();
        while let Some((i, j)) = level.pop() {
            if self.matrix[i as usize][j as usize] == '[' {
                finals.insert((i, j));
            }
            let next_i = i + di;
            if self.matrix[next_i as usize][j as usize] == '[' {
                level.push((next_i, j + 1));
                level.push((next_i, j));
                continue;
            }
            if self.matrix[next_i as usize][j as usize] == ']' {
                level.push((next_i, j));
                level.push((next_i, j - 1));
                continue;
            }
            if self.matrix[next_i as usize][j as usize] == '#' {
                return None;
            }
        }

        Some(finals)
    }

    fn run2(&mut self) -> isize {
        let (mut pos_i, mut pos_j) = self.start;
        for (_i, dir) in self.directions.iter().enumerate() {
            // println!("step {i}: {:?}", dir);

            let (di, dj) = dir.get_direction();
            let (next_i, next_j) = (pos_i + di, pos_j + dj);
            if !self.is_in_bound(next_i, next_j) {
                continue;
            }
            match self.matrix[next_i as usize][next_j as usize] {
                '#' => continue,
                '.' => {
                    self.matrix[pos_i as usize][pos_j as usize] = '.';
                    self.matrix[next_i as usize][next_j as usize] = '@';
                    (pos_i, pos_j) = (next_i, next_j);
                }
                '[' | ']' => match dir {
                    Direction::Left => {
                        for (i, j) in (0..pos_j - 2)
                            .map(|j| (pos_i as usize, j as usize))
                            .rev()
                            .collect::<Vec<(usize, usize)>>()
                        {
                            if self.matrix[i][j] == '#' {
                                break;
                            }
                            if self.matrix[i][j] == '.' {
                                for y in j..pos_j as usize {
                                    self.matrix[i][y] = self.matrix[i][y + 1]
                                }
                                self.matrix[pos_i as usize][pos_j as usize] =
                                    '.';
                                (pos_i, pos_j) = (next_i, next_j);
                                break;
                            }
                        }
                    }
                    Direction::Right => {
                        for (i, j) in (pos_j + 1..self.n)
                            .map(|j| (pos_i as usize, j as usize))
                            .collect::<Vec<(usize, usize)>>()
                        {
                            if self.matrix[i][j] == '#' {
                                break;
                            }
                            if self.matrix[i][j] == '.' {
                                for y in (pos_j as usize..=j).rev() {
                                    self.matrix[i][y] = self.matrix[i][y - 1]
                                }
                                self.matrix[pos_i as usize][pos_j as usize] =
                                    '.';
                                (pos_i, pos_j) = (next_i, next_j);
                                break;
                            }
                        }
                    }
                    Direction::Down | Direction::Up => {
                        let di = if dir == &Direction::Down { 1 } else { -1 };
                        if let Some(v) =
                            self.get_up_or_down_grid(next_i, next_j, di)
                        {
                            for (i, j) in v.clone() {
                                self.matrix[i as usize][j as usize] = '.';
                                self.matrix[i as usize][j as usize + 1] = '.';
                            }
                            for (i, j) in v {
                                self.matrix[(i + di) as usize][j as usize] =
                                    '[';
                                self.matrix[(i + di) as usize]
                                    [j as usize + 1] = ']';
                            }
                            self.matrix[pos_i as usize][pos_j as usize] = '.';
                            self.matrix[next_i as usize][next_j as usize] =
                                '@';
                            (pos_i, pos_j) = (next_i, next_j);
                        }
                    }
                },
                _ => panic!("disco!"),
            }
            // for v in self.matrix.iter() {
            //     println!("{}", v.iter().map(|n| n.to_string()).collect::<String>());
            // }
            // println!();
        }
        for v in self.matrix.iter() {
            println!(
                "{}",
                v.iter().map(|n| n.to_string()).collect::<String>()
            );
        }
        println!();

        (0..self.m)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .map(|(i, j)| {
                if self.matrix[i as usize][j as usize] == '[' {
                    return 100 * i + j;
                }
                0
            })
            .sum()
    }
}

fn part1(input: &str) -> isize {
    let mut game = Game::new(input);
    game.run()
}

fn part2(input: &str) -> isize {
    let mut game = Game::new(input);
    game.expand_matrix();
    game.run2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
            "#
        };
        assert_eq!(part1(input), 2028);
        let input = indoc! {
            r#"
            #######
            #...#.#
            #.....#
            #..OO@#
            #..O..#
            #.....#
            #######

            <vv<<^^<<^^
            "#
        };

        assert_eq!(part2(input), 618);

        let input = indoc! {
            r#"
            ##########
            #...##O..#
            #.OO.##OO#
            #.OOO.O.##
            #.#.O..O.#
            #O...@.#O#
            ##..##...#
            ##..O....#
            #....#OO.#
            ##########

            ^vvv>^>><<^^>^^^v>>^
            "#
        };
        assert_eq!(part2(input), 6358);

        let input = indoc! {
            r#"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
            "#
        };
        assert_eq!(part1(input), 10092);
        assert_eq!(part2(input), 9021);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            #######
            #.....#
            #.....#
            #.@O..#
            #..#O.#
            #...O.#
            #..O..#
            #.....#
            #######

            >><vvv>v>^^^
            "#
        };
        assert_eq!(part2(input), 1430);

        let input = indoc! {
            r#"
            ########
            #......#
            #OO....#
            #.O....#
            #.O....#
            ##O....#
            #O..O@.#
            #......#
            ########

            <^^<<>^^^<v
            "#
        };
        assert_eq!(part2(input), 2827);

        let input = indoc! {
            r#"
            ######
            #....#
            #..#.#
            #....#
            #.O..#
            #.OO@#
            #.O..#
            #....#
            ######

            <vv<<^^^
            "#
        };
        assert_eq!(part2(input), 1216);

        let input = indoc! {
            r#"
            #######
            #.....#
            #.O.O@#
            #..O..#
            #..O..#
            #.....#
            #######

            <v<<>vv<^^
            "#
        };
        assert_eq!(part2(input), 822);

        let input = indoc! {
            r#"
            #####
            #...#
            #.O@#
            #OO.#
            #O#.#
            #...#
            #####

            <^<<v
            "#
        };
        assert_eq!(part2(input), 1211);

        let input = indoc! {
            r#"
            #####
            #...#
            #.O@#
            #OO.#
            ##O.#
            #...#
            #####

            <^<<v
            "#
        };
        assert_eq!(part2(input), 1213);

        let input = indoc! {
            r#"
            #######
            #.....#
            #.....#
            #..#O.#
            #...O.#
            #...@.#
            #######

            >>^<v<<^
            "#
        };
        assert_eq!(part2(input), 715);
    }
}
