use cached::proc_macro::cached;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

#[cached(
    key = "String",
    convert = r#"{ format!("{:?}:{}", patterns, word) }"#
)]
fn word_search<'a>(patterns: &[&'a str], word: &'a str) -> usize {
    let mut total = 0;
    for pattern in patterns.iter() {
        if *pattern == word {
            total += 1;
            continue;
        }
        if pattern.len() <= word.len() && pattern[..] == word[..pattern.len()]
        {
            total += word_search(patterns, &word[pattern.len()..])
        }
    }
    total
}

fn part1(input: &str) -> usize {
    let (patterns, towels): (&str, &str) =
        input.split("\n\n").collect_tuple().unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();
    let towels: Vec<&str> = towels.lines().collect();

    towels
        .iter()
        .filter(|towel| word_search(&patterns, towel) > 0)
        .count()
}

fn part2(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
            "#
        };
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        // assert_eq!();
    }
}
