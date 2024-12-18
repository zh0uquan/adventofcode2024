use cached::proc_macro::cached;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input, 25));
    println!("{:?}", part2(input, 75));
}

fn apply_rule(n: usize) -> Vec<usize> {
    match n {
        0 => vec![1],
        n if n.to_string().len() % 2 == 0 => {
            let n_str = n.to_string();
            let mid = n_str.len() / 2;

            let (first_half, second_half) = n_str.split_at(mid);
            vec![first_half.parse().unwrap(), second_half.parse().unwrap()]
        }
        _ => vec![n * 2024],
    }
}

#[cached]
fn rec(stones: Vec<usize>, blinks_left: usize) -> usize {
    if blinks_left == 0 {
        return stones.len();
    }

    stones
        .iter()
        .map(|&stone| {
            let new_stones = apply_rule(stone);
            rec(new_stones, blinks_left - 1)
        })
        .sum()
}

fn part1(input: &str, n: usize) -> usize {
    let init: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut blink = init;
    for _i in 0..n {
        blink = blink.iter().flat_map(|n| apply_rule(*n)).collect();
    }
    blink.len()
}

fn part2(input: &str, n: usize) -> usize {
    let init: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    rec(init, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("125 17", 6), 22);
        assert_eq!(part1("125 17", 25), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("125 17", 75), 65601038650482);
    }
}
