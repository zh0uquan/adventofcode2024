use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

trait SafeCheck: Iterator<Item = isize> {
    fn safe(self) -> bool
    where
        Self: Sized,
    {
        self.tuple_windows()
            .map(|(a, b)| {
                (1 <= a.abs_diff(b) && 3 >= a.abs_diff(b), (a - b).signum())
            })
            .tuple_windows()
            .all(|(a, b)| a.0 == b.0 && a.0 && a.1 == b.1)
    }
}

impl<T> SafeCheck for T where T: Iterator<Item = isize> {}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<isize>().unwrap())
                .safe()
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let origin: Vec<isize> = line
                .split_whitespace()
                .map(|e| e.parse::<isize>().unwrap())
                .collect();
            if origin.clone().into_iter().safe() {
                return true;
            }
            for i in 0..origin.len() {
                if [&origin[..i], &origin[i + 1..]].concat().into_iter().safe()
                {
                    return true;
                }
            }
            false
        })
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
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "#
        };
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "#
        };
        assert_eq!(part2(input), 4);
    }
}
