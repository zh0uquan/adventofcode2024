use indoc::formatdoc;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::u8 as nom_u8;
use nom::character::complete::{alphanumeric1, space1};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let input = include_str!("input.txt");
    // println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn parse_value(input: &str) -> IResult<&str, (&str, u8)> {
    separated_pair(alphanumeric1, tag(": "), nom_u8)(input)
}

fn parse_wire(input: &str) -> IResult<&str, ((&str, &str, &str), &str)> {
    separated_pair(
        tuple((
            alphanumeric1,
            preceded(space1, alphanumeric1),
            preceded(space1, alphanumeric1),
        )),
        tag(" -> "),
        alphanumeric1,
    )(input)
}

struct Wire<'a> {
    a: &'a str,
    b: &'a str,
    c: &'a str,
    op: &'a str,
}

fn part1(input: &str) -> usize {
    let (values, wires) = input.split("\n\n").collect_tuple().unwrap();
    let mut values: HashMap<&str, u8> = values
        .lines()
        .map(|line| parse_value(line).unwrap().1)
        .collect();
    let mut wires: Vec<Wire> = wires
        .lines()
        .map(|line| parse_wire(line).unwrap().1)
        .map(|((a, op, b), c)| Wire { a, b, c, op })
        .collect();

    while let Some(wire) = wires.pop() {
        if let (Some(a), Some(b)) = (values.get(wire.a), values.get(wire.b)) {
            let c = match wire.op {
                "AND" => a & b,
                "XOR" => a ^ b,
                "OR" => a | b,
                _ => panic!("disco!"),
            };
            values.insert(wire.c, c);
        } else {
            wires.insert(0, wire);
        }
    }

    values
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted_by(|a, b| b.0.cmp(a.0))
        .map(|(_k, &v)| v)
        .fold(0_usize, |acc, bit| (acc << 1) | (bit as usize))
}

fn part2(input: &str) {
    let (values, wires) = input.split("\n\n").collect_tuple().unwrap();

    let values: HashMap<&str, u8> = values
        .lines()
        .map(|line| parse_value(line).unwrap().1)
        .collect();

    let initals: Vec<&str> =
        values.keys().copied().sorted_by(|a, b| b.cmp(a)).collect();

    let graph_content = wires
        .lines()
        .map(|line| parse_wire(line).unwrap().1)
        .fold(String::new(), |mut s, ((a, op, b), c)| {
            let c = match c {
                "rmj" => "z23",
                "z23" => "rmj",
                "z17" => "cmv",
                "cmv" => "z17",
                "z30" => "rdg",
                "rdg" => "z30",
                "btb" => "mwp",
                "mwp" => "btb",
                _ => c,
            };
            let color = match op {
                "AND" => "blue",
                "XOR" => "read",
                "OR" => "green",
                _ => panic!("disco!"),
            };
            s.push_str(&format!(
                "\"{}\" -> \"{}\" [label=\"{}\", color=\"{}\"]\n",
                a, c, op, color
            ));
            s.push_str(&format!(
                "\"{}\" -> \"{}\" [label=\"{}\", color=\"{}\"]\n",
                b, c, op, color
            ));
            s
        });
    let indented_graph_content = graph_content
        .lines()
        .map(|line| format!("{}{}", " ".repeat(8), line))
        .collect::<Vec<String>>()
        .join("\n");

    let graph = formatdoc! {
        "
        digraph LogicalGraph {{
            // Define graph properties
            graph [rankdir=LR];
            node [shape=ellipse, style=filled, fillcolor=lightblue];
            {graph_content}
        }}
        ", graph_content=indented_graph_content
    };

    let mut file =
        File::create("day24/graph.dot").expect("Unable to create file");
    file.write_all(graph.as_bytes())
        .expect("Unable to write to file");
    // pattern is:
    // for "Z" node => 2 black => one is from two green line and another from X AND Y
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        println!("{:?}", parse_wire("ntg XOR fgs -> mjb").unwrap());
        let input = indoc! {
            r#"
            x00: 1
            x01: 0
            x02: 1
            x03: 1
            x04: 0
            y00: 1
            y01: 1
            y02: 1
            y03: 1
            y04: 1

            ntg XOR fgs -> mjb
            y02 OR x01 -> tnw
            kwq OR kpj -> z05
            x00 OR x03 -> fst
            tgd XOR rvg -> z01
            vdt OR tnw -> bfw
            bfw AND frj -> z10
            ffh OR nrd -> bqk
            y00 AND y03 -> djm
            y03 OR y00 -> psh
            bqk OR frj -> z08
            tnw OR fst -> frj
            gnj AND tgd -> z11
            bfw XOR mjb -> z00
            x03 OR x00 -> vdt
            gnj AND wpb -> z02
            x04 AND y00 -> kjc
            djm OR pbm -> qhw
            nrd AND vdt -> hwm
            kjc AND fst -> rvg
            y04 OR y02 -> fgs
            y01 AND x02 -> pbm
            ntg OR kjc -> kwq
            psh XOR fgs -> tgd
            qhw XOR tgd -> z09
            pbm OR djm -> kpj
            x03 XOR y03 -> ffh
            x00 XOR y04 -> ntg
            bfw OR bqk -> z06
            nrd XOR fgs -> wpb
            frj XOR qhw -> z04
            bqk OR frj -> z07
            y03 OR x01 -> nrd
            hwm AND bqk -> z03
            tgd XOR rvg -> z12
            tnw OR pbm -> gnj
            "#
        };
        assert_eq!(part1(input), 2024);
    }
}
