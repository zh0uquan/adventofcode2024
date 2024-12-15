use nom::character::complete::i64 as nom_i64;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

#[derive(Debug)]
struct ButtonPrize {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_button(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("X+"), nom_i64),
        tag(", "),
        preceded(tag("Y+"), nom_i64),
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("X="), nom_i64),
        tag(", "),
        preceded(tag("Y="), nom_i64),
    )(input)
}

// Parse each button-prize block (Button A, Button B, Prize) and terminated by newline
fn parse_button_prize(input: &str) -> IResult<&str, ButtonPrize> {
    let (input, button_a) =
        terminated(preceded(tag("Button A: "), parse_button), newline)(input)?;
    let (input, button_b) =
        terminated(preceded(tag("Button B: "), parse_button), newline)(input)?;
    let (input, prize) = preceded(tag("Prize: "), parse_prize)(input)?;

    Ok((
        input,
        ButtonPrize {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn solve_linear_system(
    a1: i64,
    a2: i64,
    b1: i64,
    b2: i64,
    c1: i64,
    c2: i64,
) -> Option<(i64, i64)> {
    let denominator = a1 * b2 - a2 * b1;
    if denominator == 0 {
        return None;
    }
    let numerator_x = b2 * c1 - b1 * c2;
    let numerator_y = a1 * c2 - a2 * c1;

    if numerator_x % denominator == 0 && numerator_y % denominator == 0 {
        Some((numerator_x / denominator, numerator_y / denominator))
    } else {
        None
    }
}

fn part1(input: &str) -> i64 {
    let (_input, button_prizes) =
        separated_list1(tag("\n\n"), parse_button_prize)(input).unwrap();
    button_prizes
        .iter()
        .filter_map(|button_prize| {
            solve_linear_system(
                button_prize.button_a.0,
                button_prize.button_a.1,
                button_prize.button_b.0,
                button_prize.button_b.1,
                button_prize.prize.0,
                button_prize.prize.1,
            )
        })
        .map(|(a, b)| {
            // println!("{a}, {b}");
            a * 3 + b
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let (_input, button_prizes) =
        separated_list1(tag("\n\n"), parse_button_prize)(input).unwrap();
    button_prizes
        .iter()
        .filter_map(|button_prize| {
            solve_linear_system(
                button_prize.button_a.0,
                button_prize.button_a.1,
                button_prize.button_b.0,
                button_prize.button_b.1,
                button_prize.prize.0 + 10000000000000,
                button_prize.prize.1 + 10000000000000,
            )
        })
        .map(|(a, b)| {
            // println!("{a}, {b}");
            a * 3 + b
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
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
            "#
        };
        assert_eq!(part1(input), 480);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
            "#
        };
        assert_eq!(part2(input), 875318608908);
    }
}
