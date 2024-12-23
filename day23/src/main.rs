use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn get_computer_networks(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .lines()
        .map(|line| line.split('-').collect_tuple().unwrap())
        .fold(HashMap::new(), |mut map, (a, b)| {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
            map
        })
}

fn greedy_clique_approximation<'a>(
    graph: &'a HashMap<&str, HashSet<&str>>,
) -> HashSet<&'a str> {
    let mut vertices: Vec<_> = graph.keys().collect();
    let mut rng = thread_rng();
    vertices.shuffle(&mut rng);
    vertices.sort_by_key(|&v| -(graph[v].len() as isize));

    let mut clique: HashSet<&str> = HashSet::new();

    for &vertex in vertices.iter() {
        if clique.iter().all(|&member| graph[member].contains(vertex)) {
            clique.insert(vertex);
        }
    }

    clique
}

fn part1(input: &str) -> usize {
    let internets: HashSet<(&str, &str, &str)> = get_computer_networks(input)
        .iter()
        .combinations(2)
        .filter(|pair| pair[0].1.contains(pair[1].0))
        .flat_map(|pair| {
            let (a, na) = pair[0];
            let (b, nb) = pair[1];

            na.intersection(nb).map(|&common| {
                let mut members = [a, b, common];
                members.sort_unstable();
                (members[0], members[1], members[2])
            })
        })
        .collect();
    internets
        .iter()
        .filter(|&&tuple| {
            tuple.0.starts_with('t')
                || tuple.1.starts_with('t')
                || tuple.2.starts_with('t')
        })
        .count()
}

fn part2(input: &str) -> String {
    let graph = get_computer_networks(input);
    let mut max_guess = String::new();
    for _ in 0..1000 {
        let largest_set = greedy_clique_approximation(&graph);
        let guess = largest_set.iter().sorted().join(",");
        if guess.len() > max_guess.len() {
            max_guess = guess.to_string();
        }
    }
    max_guess
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {
            r#"
            kh-tc
            qp-kh
            de-cg
            ka-co
            yn-aq
            qp-ub
            cg-tb
            vc-aq
            tb-ka
            wh-tc
            yn-cg
            kh-ub
            ta-co
            de-co
            tc-td
            tb-wq
            wh-td
            ta-ka
            td-qp
            aq-cg
            wq-ub
            ub-vc
            de-ta
            wq-aq
            wq-vc
            wh-yn
            ka-de
            kh-ta
            co-tc
            wh-qp
            tb-vc
            td-yn
            "#
        };
        part1(input);
        assert_eq!(part1(input), 7);
        assert_eq!(part2(input), "co,de,ka,ta");
    }
}
