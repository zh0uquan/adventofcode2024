use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn get_following_map(first_section: &str) -> HashMap<&str, HashSet<&str>> {
    first_section
        .lines()
        .map(|line| line.split('|').collect_tuple().unwrap())
        .fold(HashMap::new(), |mut map, (before, after)| {
            map.entry(before).or_default().insert(after);
            map
        })
}
fn part1(input: &str) -> usize {
    let (first_section, second_section): (&str, &str) =
        input.split("\n\n").collect_tuple().unwrap();

    let following_map = get_following_map(first_section);

    second_section
        .lines()
        .map(|line| {
            let nums: Vec<&str> = line.split(',').collect();
            let mid = nums[nums.len() / 2];
            let (correct, _) = nums.iter().fold(
                (true, HashSet::new()),
                |(mut correct, mut befores), num| {
                    if !correct {
                        return (correct, befores);
                    }
                    if let Some(set) = following_map.get(num) {
                        if !befores.is_disjoint(set) {
                            correct = false;
                            return (correct, befores);
                        }
                    }
                    befores.insert(num);
                    (correct, befores)
                },
            );
            (mid, correct)
        })
        .filter_map(|(mid, correct)| {
            if correct {
                return Some(mid.parse::<usize>().unwrap());
            }
            None
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let (first_section, second_section): (&str, &str) =
        input.split("\n\n").collect_tuple().unwrap();

    let following_map = get_following_map(first_section);

    second_section
        .lines()
        .map(|line| {
            let nums: Vec<&str> = line.split(',').collect();
            let mut nums_cloned = nums.clone();
            nums_cloned.sort_by(|a, b| {
                match following_map
                    .get(*a)
                    .unwrap_or(&HashSet::new())
                    .contains(b)
                {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            });
            if nums_cloned == nums {
                return 0;
            }
            nums_cloned[nums_cloned.len() / 2].parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            "#
        };
        assert_eq!(part1(input), 143);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            "#
        };
        assert_eq!(part2(input), 123);
    }
}
