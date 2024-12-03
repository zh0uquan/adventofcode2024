use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = input.lines()
        .map(|line| 
            line.split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        )
        .unzip();
    
    left.sort();
    right.sort();
    left.iter().zip(&right).map(|(a, b)| a.abs_diff(*b) ).sum()
}


fn part2(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = input.lines()
        .map(|line| 
            line.split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        )
        .unzip();
    
    let counts = right.iter().counts();
    left.iter().map(|a| counts.get(a).unwrap_or(&0) * *a).sum()
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#
        };
        assert_eq!(part1(input), 11);
    }


    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#
        };
        assert_eq!(part2(input), 31);
    }
}