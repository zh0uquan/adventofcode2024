use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input));
}


fn is_loop(matrix: Vec<Vec<char>>, mut pos_i: usize, mut pos_j: usize, mut di: isize, mut dj: isize) -> bool {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut visited = HashSet::new();
    while let (Some(i), Some(j)) = (pos_i.checked_add_signed(di), pos_j.checked_add_signed(dj)) {
        if visited.contains(&(pos_i, pos_j, di, dj)) {
            return true;
        }
        if i == m || j == n {
            return false;
        }
        visited.insert((pos_i, pos_j, di, dj));
        match matrix[i][j] { 
            '#' => {
                (di, dj) = match (di, dj) {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => panic!("disco!")
                };
            },
            '.' | '^' => {
                (pos_i, pos_j) = (i, j);
            },
            _ => panic!("disco!")
        }
    }
    false
}

fn solve(input: &str) -> (usize, usize) {
    let matrix: Vec<Vec<char>> = input.lines().map(
        |line| line.chars().collect()
    ).collect();

    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut start_i, mut start_j): (usize, usize) = (0, 0);
    let (mut di, mut dj): (isize, isize) = (-1, 0);
    let mut visited = HashSet::new();
    
    for (i, j) in (0..m).flat_map(|i| (0..n).map(move |j| (i, j))) {
        if matrix[i][j] == '^' {
            (start_i, start_j) = (i, j);
        }
    }
    
    let (mut pos_i, mut pos_j): (usize, usize) = (start_i, start_j);
    while let (Some(i), Some(j)) = (pos_i.checked_add_signed(di), pos_j.checked_add_signed(dj)) {
        if i == m || j == n {
            break;
        }
        visited.insert((pos_i, pos_j));
        match matrix[i][j] { 
            '#' => {
                (di, dj) = match (di, dj) {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => panic!("disco!")
                };
            },
            '.' | '^' => {
                (pos_i, pos_j) = (i, j);
            },
            _ => panic!("disco!")
        }
    }

    let obs: HashSet<(usize, usize)> = visited.clone().into_iter().filter(
        |(obs_i, obs_j)| {
            let mut matrix_clone = matrix.clone();
            matrix_clone[*obs_i][*obs_j] = '#';
            is_loop(
                matrix_clone, start_i, start_j, -1, 0
            ) 
        }
    ).collect();
    
    (visited.len() + 1, obs.len() + 1)
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {
            r#"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            "#
        };
        assert_eq!(solve(input), (41, 6))   
    }
}
