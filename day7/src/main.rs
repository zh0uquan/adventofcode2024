use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}


fn part1(input: &str) -> usize {
    input.lines().map(
        |line| {
            let (value, nums): (&str, &str) = line.split(": ").collect_tuple().unwrap();
            let value = value.parse::<usize>().unwrap();
            let nums: Vec<usize> = nums.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
            let evals = nums.iter().fold(vec![], |acc, &n| {
                if acc.is_empty() {
                    vec![n]
                } else {
                    acc.iter().flat_map(|&prev| {
                        vec![
                            prev * n,
                            prev + n,
                        ]
                    }).collect()
                }
            });
            if evals.contains(&value) {
                return value
            }
            0
        }
    ).sum()
    
}

fn part2(input: &str) -> usize {
    input.lines().map(
        |line| {
            let (value, nums): (&str, &str) = line.split(": ").collect_tuple().unwrap();
            let value = value.parse::<usize>().unwrap();
            let nums: Vec<usize> = nums.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
            let evals = nums.iter().fold(vec![], |acc, &n| {
                if acc.is_empty() {
                    vec![n]
                } else {
                    acc.iter().flat_map(|&prev| {
                        vec![
                            prev * n,
                            prev + n,
                            (prev.to_string() + &n.to_string()).parse().unwrap()
                        ]
                    }).collect()
                }
            });
            if evals.contains(&value) {
                return value
            }
            0
        }
    ).sum()
    
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            "#
        };
        assert_eq!(part1(input), 3749);
    }


    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            "#
        };
        assert_eq!(part2(input), 11387);
        
    }
}
