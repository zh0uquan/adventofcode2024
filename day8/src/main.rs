use std::collections::{HashMap, HashSet};
use itertools::{iproduct};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input));
}

fn mirror_point(point: (usize, usize), reference: (usize, usize)) -> Option<(usize, usize)> {
    let (x, y) = point;
    let (rx, ry) = reference;

    if 2 * rx >= x && 2 * ry >= y {
        return Some((2 * rx - x, 2 * ry - y));
    } 
    None
}

fn mirror_point_within_grid(
    point: (usize, usize),
    reference: (usize, usize),
    grid: (usize, usize),
) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let (rx, ry) = reference;
    let (m, n) = grid;
    let mut points = vec![];

    if let (Some(dx), Some(dy)) = (
        (x as isize).checked_sub(rx as isize),
        (y as isize).checked_sub(ry as isize),
    ) {
        let mut current_rx = rx as isize;
        let mut current_ry = ry as isize;

        while let (Ok(new_x), Ok(new_y)) = (
            (current_rx + dx).try_into(),
            (current_ry + dy).try_into(),
        ) {
            if new_x >= m || new_y >= n {
                break;
            }
            points.push((new_x, new_y));
            current_rx += dx;
            current_ry += dy;
        }
    }

    points
}



fn solve(input: &str) -> (usize, usize) {
    let matrix: Vec<Vec<char>> = input.lines().map(
        |line| line.chars().collect()
    ).collect();
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut antennas_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, j) in (0..m).flat_map(|i| (0..n).map(move |j| (i, j))) {
        if matrix[i][j] != '.' {
            antennas_map.entry(matrix[i][j]).or_default().push(
                (i, j)
            );
        }
    }
    let mut antinodes = HashSet::new();
    let mut antinodes_updated = HashSet::new();
    for pairs in antennas_map.values() {
        for (x, y) in iproduct!(pairs.iter(), pairs.iter()).filter(|&(x, y)| x != y) {
            if let Some((mirror_x, mirror_y)) = mirror_point(*x, *y) {
                if mirror_x < m && mirror_y < n {
                    antinodes.insert((mirror_x, mirror_y));
                }
            }
            antinodes_updated.extend(
                mirror_point_within_grid(*x, *y, (m,n))
            );
        }
    }
    // println!("{:?} {:?}", antinodes_updated, antinodes_updated.len());
    (antinodes.len(), antinodes_updated.len())
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            "#
        };
        assert_eq!(solve(input), (14, 34));
    }


    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            T.........
            ...T......
            .T........
            ..........
            ...x.......
            ..........
            ..........
            ..........
            ..........
            ..........
            "#
        };
        solve(input);
        // assert_eq!();
    }
}
