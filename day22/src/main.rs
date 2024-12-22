use std::arch::aarch64::vabsq_f32;
use std::collections::{HashMap};
use cached::proc_macro::cached;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input, 2000));
}

fn mix(secret: i64, value: i64) -> i64 {
    secret ^ value
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

#[cached]
fn evolve_secret(mut secret: i64) -> i64 {
    // Step 1: Multiply by 64
    let result = secret * 64;
    secret = mix(secret, result);
    secret = prune(secret);

    // Step 2: Divide by 32 and round down
    let result = secret / 32;
    secret = mix(secret, result);
    secret = prune(secret);

    // Step 3: Multiply by 2048
    let result = secret * 2048;
    secret = mix(secret, result);
    secret = prune(secret);

    secret
}

#[cached]
fn evolve_n_times(secret: i64, n: usize) -> i64 {
    if n == 0 {
        return secret
    }
    return evolve_n_times(evolve_secret(secret), n - 1);
}

fn take_last(secret: i64) -> i64 {
    secret.to_string().chars().last().unwrap() as i64 - b'0' as i64
}

fn part1(input: &str) -> i64 {
    input.lines().map(|line| evolve_n_times(line.parse::<i64>().unwrap(), 2000)).sum()
}

fn part2(input: &str, n: usize) -> i64 {
    let mut total = vec![];
    for line in input.lines() {
        let initial = line.parse::<i64>().unwrap();
        let prices: Vec<i64> = (0..n)
            .fold(vec![initial], |mut acc, _n| {
                acc.push(evolve_secret(acc[acc.len() - 1]));
                acc
            })
            .into_iter()
            .map(take_last)
            .collect();
        let changes: Vec<i64> = prices.iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        let sequences = (0..changes.len() - 3).fold(
            HashMap::new(),
            |mut sequences, i| {
                let key = format!("{:?}", &changes[i..i+4]);
                if sequences.contains_key(&key) {
                    return sequences;
                }
                sequences.insert(key, prices[i+4]);
                sequences
            }
        );
        total.push(sequences);
        // println!("{:?}", sequences)
    }
    println!("finished sequences!");
    let counter: HashMap<String, i64> = total.iter().fold(
        HashMap::new(), |mut counter, sequences| {
            for (key, value) in sequences {
                *counter.entry(key.clone()).or_default() += value;
            }
            counter
        }
    );
    // println!("{:?}", counter);
    
    *counter.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_evolution() {
        let secret = 42;
        let evolved = evolve_secret(secret);
        assert!(evolved < 16777216);
    }

    #[test]
    fn test_evolution_n_times() {
        let secret = 123;
        let evolved = evolve_n_times(secret, 10);
        assert_eq!(evolved, 5908254);
    }

    #[test]
    fn test_take_last() {
        let secret = 15887950;
        assert_eq!(take_last(secret), 0);
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            1
            10
            100
            2024
            "#
        };
        assert_eq!(part1(input), 37327623);
    }


    #[test]
    fn test_part2() {
        assert_eq!(part2("123", 10), 6);


        let input = indoc! {
            r#"
            1
            2
            3
            2024
            "#
        };
        assert_eq!(part2(input, 2000), 23);

    }
}
