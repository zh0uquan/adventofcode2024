use cached::proc_macro::cached;
use common::{Coord, Direction, Matrix};
use indoc::indoc;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input, 1));
    println!("{:?}", solve(input, 24));
}

const EMPTY: char = '.';
const CONFIRM: char = 'A';

const NUMERIC_PAD: &str = indoc! {
    r#"
       789
       456
       123
       .0A
    "#
};

const CONTROL_PAD: &str = indoc! {
    r#"
    .^A
    <v>
    "#
};

struct Solver {
    numeric_keypad: Matrix<char>,
    control_keypad: Matrix<char>,
}

impl Solver {
    
    fn get_min_move(&self, possible_moves: Vec<Vec<Direction>>) -> String {
        let possible_ch_moves: Vec<String> = possible_moves
            .iter()
            .map(|moves| {
                moves
                    .iter()
                    .map(|&dir| self.numeric_keypad.convert_direction(dir))
                    .collect::<String>()
            })
            .collect();
        if possible_ch_moves.is_empty() {
            return String::new();
        }
        println!("{:?}", possible_ch_moves);
        possible_ch_moves
            .iter()
            .max_by_key(|moves| {
                let contiguous_count = moves
                    .as_bytes()
                    .windows(2)
                    .filter(|pair| pair[0] == pair[1])
                    .count();
                contiguous_count
            })
            .cloned()
            .unwrap()
    }

    fn _interpret_two_pos(
        &self,
        input: &str,
        keypad: &Matrix<char>,
    ) -> String {
        // from '>' to '^'  :  only possible way is <^A
        // from '^' to '>'  :  only possible way is v>A
        // from 'A' to 'v'  :  only possible way is <vA
        // from 'v' to 'A'  :  only possible way is ^>A
        // hardcode way to do this, very bad way!!!
        if input == ">^" {
            return "<^".to_string();
        }
        if input == "^>" {
            return "v>".to_string();
        }
        if input == "Av" {
            return "<v".to_string();
        }
        if input == "vA" {
            return "^>".to_string()
        }
        
        
        let coord_a = keypad.get_pos(input.as_bytes()[0] as char).unwrap();
        let coord_b = keypad.get_pos(input.as_bytes()[1] as char).unwrap();
        let possible_moves = keypad.find_shortest_moves(coord_a, coord_b);
        self.get_min_move(possible_moves)
    }
    fn _interpret(&self, input: &str, keypad: &Matrix<char>) -> String {
        let mut controls: Vec<char> = input.chars().collect();
        // println!("{:?}", controls.iter().collect::<String>());

        let mut cached: HashMap<String, String> = HashMap::new();
        controls.insert(0, CONFIRM);
        let result: String = controls
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| {
                self._interpret_two_pos(&format!("{a}{b}"), keypad)
            })
            .map(|mut moves| {
                moves.push(CONFIRM);
                moves
            })
            .collect();
        cached.insert(input.to_string(), result.clone());
        result
    }

    fn interpret_control(&self, input: &str) -> String {
        self._interpret(input, &self.control_keypad)
    }

    fn interpret_code(&self, input: &str) -> String {
        self._interpret(input, &self.numeric_keypad)
    }
}

trait KeyPad {
    fn get_pos(&self, ch: char) -> Option<Coord>;
    fn find_shortest_moves(
        &self,
        start: Coord,
        end: Coord,
    ) -> Vec<Vec<Direction>>;
    fn convert_direction(&self, dir: Direction) -> char;
}

impl KeyPad for Matrix<char> {
    fn get_pos(&self, ch: char) -> Option<Coord> {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.matrix[i][j] == ch {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn find_shortest_moves(
        &self,
        start: Coord,
        end: Coord,
    ) -> Vec<Vec<Direction>> {
        if start == end {
            return vec![];
        }
        let mut queue = BinaryHeap::new();
        let mut distance: HashMap<Coord, usize> = HashMap::new();
        let mut moves = vec![];
        queue.push(Reverse((0, start, vec![])));
        while let Some(Reverse((curr_score, curr_pos, curr_moves))) =
            queue.pop()
        {
            if *distance.get(&curr_pos).unwrap_or(&usize::MAX) < curr_score {
                continue;
            }
            distance.insert(curr_pos, curr_score);
            if curr_pos == end {
                moves.push(curr_moves.clone());
            }
            for (next_pos, dir) in self.get_coord_neighbours(curr_pos) {
                if *distance.get(&next_pos).unwrap_or(&usize::MAX) > curr_score
                    && self[next_pos] != EMPTY
                {
                    let mut curr_moves_clone = curr_moves.clone();
                    curr_moves_clone.push(dir);
                    queue.push(Reverse((
                        curr_score + 1,
                        next_pos,
                        curr_moves_clone,
                    )));
                }
            }
        }
        moves
    }

    fn convert_direction(&self, direction: Direction) -> char {
        match direction {
            (-1, 0) => '^',
            (1, 0) => 'v',
            (0, 1) => '>',
            (0, -1) => '<',
            _ => panic!("disco"),
        }
    }
}

fn solve(input: &str, n: usize) -> usize {
    let numeric_keypad = Matrix::from(NUMERIC_PAD, |c: char| c);
    let control_keypad = Matrix::from(CONTROL_PAD, |c: char| c);
    let solver = Solver {
        numeric_keypad,
        control_keypad,
    };
    #[cached(key = "String", convert = r#"{ format!("{} {}", input, times) }"#)]
    fn recursion(
        input: &str,
        times: usize,
        solver: &Solver,
    ) -> usize {
        let mut total = 0;
        let input = if times != 0 {
            format!("{}{}", CONFIRM, input)
        } else { 
            input.to_string()
        };
        if times == 0 {
            let last = &solver._interpret(&input, &solver.control_keypad);
            println!("last: {} times: {}", last, times);
            return last.len();
        }
        for index in 0..input.len() - 1 {
            let go = &input[index..=index + 1];
            let go = &format!(
                "{}{}",
                &solver
                    ._interpret_two_pos(go, &solver.control_keypad),
                CONFIRM
            );
            println!("input: {input}, times: {times}, go: {go}");
            total += &recursion(go, times - 1, solver)
        }
        total
    }
    
    let nums: Vec<usize> = input
        .lines()
        .map(|line| {
            recursion(&solver.interpret_code(line), n, &solver)
        })
        .collect();
    println!("{:?}", nums);
    input
        .lines()
        .zip(nums)
        .map(|(line, length)| {
            let num = line
                .strip_suffix(CONFIRM)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            num * length
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_keypads() {
        let numeric_keypad = Matrix::from(NUMERIC_PAD, |c: char| c);
        let control_keypad = Matrix::from(CONTROL_PAD, |c: char| c);
        let solver = Solver {
            numeric_keypad,
            control_keypad,
        };

        assert_eq!(
            solver.interpret_code("029A"),
            "<A^A>^^AvvvA"
        );
        assert_eq!(
            solver.interpret_control("<A^A>^^AvvvA"),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
        );
        assert_eq!(
            solver.interpret_control("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
        );
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            029A
            980A
            179A
            456A
            379A
            "#
        };
        assert_eq!(solve(input, 1), 126384);
        assert_eq!(solve(input, 24), 154154076501218)
    }
}