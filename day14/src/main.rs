use nom::bytes::complete::tag;
use nom::character::complete::{i64 as nom_i64, space1, u64 as nom_u64};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input, 103, 101));
    println!("{:?}", part2(input, 103, 101));
}

#[derive(Debug, Default, Copy, Clone)]
struct Robot {
    pos: (u64, u64),
    velocity: (i64, i64),
}

fn parse_coord(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(nom_u64, tag(","), nom_u64)(input)
}

fn parse_velocity(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(nom_i64, tag(","), nom_i64)(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, ((j, i), (vj, vi))) = separated_pair(
        preceded(tag("p="), parse_coord),
        space1,
        preceded(tag("v="), parse_velocity),
    )(input)?;
    Ok((
        input,
        Robot {
            pos: (i, j),
            velocity: (vi, vj),
        },
    ))
}

fn part1(input: &str, m: usize, n: usize) -> usize {
    let mut matrix: Vec<Vec<usize>> =
        (0..m).map(|_i| (0..n).map(|_j| 0).collect()).collect();

    input
        .lines()
        .map(|line| {
            let (_input, robot) = parse_robot(line).unwrap();
            robot
        })
        .for_each(|robot| {
            let (ti, tj) = (
                robot.pos.0 as i64 + robot.velocity.0 * 100,
                robot.pos.1 as i64 + robot.velocity.1 * 100,
            );
            let (mut res_i, mut res_j) = (ti % m as i64, tj % n as i64);
            if res_i < 0 {
                res_i += m as i64;
            }
            if res_j < 0 {
                res_j += n as i64;
            }
            matrix[res_i as usize][res_j as usize] += 1;
        });

    // for v in matrix.iter() {
    //     println!("{}", v.iter().map(|n| n.to_string()).collect::<String>());
    // }
    [
        (0..m / 2, n / 2 + 1..n),
        (0..m / 2, 0..n / 2),
        (m / 2 + 1..m, 0..n / 2),
        (m / 2 + 1..m, n / 2 + 1..n),
    ]
    .into_iter()
    .map(|(rows, cols)| {
        rows.map(|i| cols.clone().map(|j| matrix[i][j]).sum::<usize>())
            .sum::<usize>()
    })
    .product()
}

fn part2(input: &str, m: usize, n: usize) -> io::Result<()> {
    let matrix: Vec<Vec<char>> =
        (0..m).map(|_i| (0..n).map(|_j| ' ').collect()).collect();

    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let (_input, robot) = parse_robot(line).unwrap();
            robot
        })
        .collect();
    let mut file = File::create("christmas").unwrap();

    for i in 0..100000 {
        writeln!(file, "----------------------------------------------")?;
        writeln!(file, "times: {i}")?;
        let mut matrix_cloned = matrix.clone();
        for robot in robots.iter() {
            let (ti, tj) = (
                robot.pos.0 as i64 + robot.velocity.0 * i as i64,
                robot.pos.1 as i64 + robot.velocity.1 * i as i64,
            );
            let (mut res_i, mut res_j) = (ti % m as i64, tj % n as i64);
            if res_i < 0 {
                res_i += m as i64;
            }
            if res_j < 0 {
                res_j += n as i64;
            }
            matrix_cloned[res_i as usize][res_j as usize] = 'X';
        }
        for v in matrix_cloned.iter() {
            writeln!(
                file,
                "{}",
                v.iter().map(|n| n.to_string()).collect::<String>()
            )?;
        }
        writeln!(file, "----------------------------------------------")?;
        writeln!(file)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
            "#
        };
        assert_eq!(part1(input, 7, 11), 12);
    }

    #[test]
    fn test_part2() {
        // assert_eq!();
    }
}
