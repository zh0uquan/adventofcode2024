use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input));
}

struct Matrix {
    grid: Vec<Vec<u32>>,
    m: usize,
    n: usize,
}

impl Matrix {
    fn get_trailheads(&self) -> Vec<(usize, usize)> {
        let mut trailheads = vec![];
        for (i, j) in
            (0..self.m).flat_map(|i| (0..self.n).map(move |j| (i, j)))
        {
            if self.grid[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
        trailheads
    }

    fn compute_score(&self) -> (usize, usize) {
        fn score(
            pos_i: usize,
            pos_j: usize,
            height: u32,
            visited: &mut Vec<(usize, usize)>,
            matrix: &Matrix,
            top: &mut HashSet<(usize, usize)>,
        ) -> usize {
            visited.push((pos_i, pos_j));
            if height == 9 {
                println!(
                    "{:?}",
                    visited
                        .iter()
                        .map(|(i, j)| format!(
                            "{}({i}, {j})",
                            matrix.grid[*i][*j]
                        ))
                        .collect::<Vec<String>>()
                        .join("=>")
                );
                if !top.contains(&(pos_i, pos_j)) {
                    top.insert((pos_i, pos_j));
                }
                return 1;
            }
            let mut total = 0;
            for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if let (Some(i), Some(j)) = (
                    pos_i.checked_add_signed(di),
                    pos_j.checked_add_signed(dj),
                ) {
                    if i < matrix.m
                        && j < matrix.n
                        && matrix.grid[i][j] > height
                        && matrix.grid[i][j] - height == 1
                        && !visited.contains(&(i, j))
                    {
                        total += score(i, j, height + 1, visited, matrix, top);
                        visited.pop();
                    }
                }
            }
            total
        }

        self.get_trailheads()
            .iter()
            .map(|(i, j)| {
                let mut top = HashSet::new();
                let total = score(*i, *j, 0, &mut vec![], self, &mut top);
                (top.len(), total)
            })
            .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y))
    }
}

fn solve(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (m, n) = (grid.len(), grid[0].len());
    let matrix = Matrix { grid, m, n };
    matrix.compute_score()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
            "#
        };
        assert_eq!(solve(input), (36, 81));
    }
}
