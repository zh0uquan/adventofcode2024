use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    re.captures_iter(input)
        .map(|cap| {
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();
            a * b
        })
        .sum()

}
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't|do"  ).expect("Invalid regex");

    re.captures_iter(input).fold(
        (0, true), |(mut sum, mut active), cap| {
        match &cap[0] {
            "don't" => active = false,
            "do" => active = true,
            _ if active => {
                let (a, b): (u32, u32) = (
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                );
                sum += a * b;
            }
            _ => {
                // do nothing
            },
        }
        (sum, active)
    }).0
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
            "#
        };
        assert_eq!(part1(input), 161);
    }


    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
           "#
        };
        assert_eq!(part2(input), 48);
    }
}