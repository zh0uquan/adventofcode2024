use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input));
}

fn find_islands(
    grid: &[Vec<char>],
) -> HashMap<char, Vec<Vec<(usize, usize)>>> {
    let mut visited = HashSet::new();
    let mut islands: HashMap<char, Vec<Vec<(usize, usize)>>> = HashMap::new();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if visited.contains(&(i, j)) {
                continue;
            }

            let ch = grid[i][j];
            let mut island = Vec::new();
            let mut stack = vec![(i, j)];

            while let Some((x, y)) = stack.pop() {
                if x >= rows
                    || y >= cols
                    || visited.contains(&(x, y))
                    || grid[x][y] != ch
                {
                    continue;
                }
                visited.insert((x, y));
                island.push((x, y));
                for &(dx, dy) in &directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && ny >= 0 {
                        stack.push((nx as usize, ny as usize));
                    }
                }
            }

            if !island.is_empty() {
                islands.entry(ch).or_default().push(island);
            }
        }
    }

    islands
}

fn calculate_perimeter(
    island: &Vec<(usize, usize)>,
    m: usize,
    n: usize,
) -> usize {
    let mut perimeter = 0;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(i, j) in island {
        for (di, dj) in &directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni < 0
                || ni >= m as isize
                || nj < 0
                || nj >= n as isize
                || !island.contains(&(ni as usize, nj as usize))
            {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn count_corners(i: isize, j: isize, island: &[(usize, usize)]) -> usize {
    let island: Vec<(isize, isize)> = island
        .iter()
        .map(|(x, y)| (*x as isize, *y as isize))
        .collect();
    [(1, -1), (1, 1), (-1, -1), (-1, 1)]
        .iter()
        .filter(|&(di, dj)| {
            let corner_1 = (i, j + dj);
            let corner_2 = (i + di, j);
            let corner_3 = (i + di, j + dj);

            (!island.contains(&corner_1) && !island.contains(&corner_2))
                || (island.contains(&corner_1)
                    && island.contains(&corner_2)
                    && !island.contains(&corner_3))
        })
        .count()
}

fn solve(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (grid.len(), grid[0].len());
    let islands = find_islands(&grid);
    println!("{:?}", islands);

    let part1 = islands
<<<<<<< Updated upstream
        .values()
        .map(|islands| {
=======
        .iter()
        .map(|(_ch, islands)| {
>>>>>>> Stashed changes
            islands
                .iter()
                .map(move |island| {
                    let perimeter = calculate_perimeter(island, m, n);
                    // println!("{ch}: {} * {:?}", island.len(), perimeter);
                    perimeter * island.len()
                })
                .sum::<usize>()
        })
        .sum();

    let part2 = islands
        .iter()
        .map(|(ch, islands)| {
            islands
                .iter()
                .map(|island| {
                    let corners = island
                        .iter()
                        .map(|(i, j)| {
                            count_corners(*i as isize, *j as isize, island)
                        })
                        .sum::<usize>();
                    println!("{ch}: {} * {:?}", island.len(), corners);
                    corners * island.len()
                })
                .sum::<usize>()
        })
        .sum();

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
            AAAA
            BBCD
            BBCC
            EEEC
            "#
        };
        assert_eq!(solve(input), (140, 80));

        let input = indoc! {
            r#"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
            "#
        };
        assert_eq!(solve(input).0, 1930);
    }
}
