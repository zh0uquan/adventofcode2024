fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

trait XMAS {
    fn exist(&self) -> usize {
        0
    }
}

impl XMAS for Vec<char> {
    fn exist(&self) -> usize {
        let s = self.iter().collect::<String>();
        let rev_s = self.iter().rev().collect::<String>();
        s.matches("XMAS").count() + rev_s.matches("XMAS").count()
    }
}

fn count(matrix: Vec<Vec<char>>) -> usize {
    matrix.iter().map(|v| v.exist()).sum()
}

fn part1(input: &str) -> usize {
    let matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let n = matrix.len();
    let vertical_matrix: Vec<Vec<char>> = (0..n)
        .map(|j| (0..n).map(|i| matrix[i][j]).collect())
        .collect();
    // println!("{:?}", vertical_matrix);

    let diagonal_matrix: Vec<Vec<char>> =
        (0..n)
            .map(|d| (0..=d).map(|i| matrix[i][d - i]).collect())
            .chain((1..n).map(|d| {
                (0..n - d).map(|i| matrix[d + i][n - 1 - i]).collect()
            }))
            .collect();
    // println!("{:?}", diagonal_matrix);

    let anti_diagonal_matrix: Vec<Vec<char>> = (0..2 * n - 1)
        .map(|diagonal| {
            (0..n)
                .filter_map(|row| {
                    let col = diagonal as i32 - row as i32;
                    if col >= 0 && col < n as i32 {
                        Some(matrix[n - 1 - row][col as usize])
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    // println!("{:?}", anti_diagonal_matrix);

    count(matrix)
        + count(vertical_matrix)
        + count(diagonal_matrix)
        + count(anti_diagonal_matrix)
}

fn part2(input: &str) -> usize {
    let matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let n = matrix.len();
    let cross = [('M', 'S'), ('S', 'M')];
    (1..n - 1)
        .flat_map(|i| (1..n - 1).map(move |j| (i, j)))
        .map(|(i, j)| {
            if matrix[i][j] != 'A' {
                return false;
            }
            if cross.contains(
                &(matrix[i - 1][j - 1], matrix[i + 1][j + 1])
            ) && cross.contains(
                &(matrix[i - 1][j + 1], matrix[i + 1][j - 1])
            ) {
                return true;
            }
            false
        })
        .filter(|&b| b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            "#
        };
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            "#
        };
        assert_eq!(part2(input), 9);
    }
}
