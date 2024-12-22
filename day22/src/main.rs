use cached::proc_macro::cached;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

#[cached]
fn evolve_secret(mut secret: u64) -> u64 {
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
fn evolve_n_times(secret: u64, n: usize) -> u64 {
    if n == 0 {
        return secret
    }
    return evolve_n_times(evolve_secret(secret), n - 1);
}

fn part1(input: &str) -> u64 {
    input.lines().map(|line| evolve_n_times(line.parse::<u64>().unwrap(), 2000)).sum()
}

fn part2(input: &str) {}

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
        // assert_eq!();
    }
}
