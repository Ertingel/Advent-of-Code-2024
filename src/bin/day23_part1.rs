// cargo run  --bin day23_part1
// cargo test --bin day23_part1

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../././input/day23.txt");
    let output = solve(input);
    println!("Day23 part1: {output}");
}

type Computer<'a> = &'a str;

fn parse_input(input: &str) -> HashMap<Computer, Vec<Computer>> {
    let mut out: HashMap<Computer, Vec<Computer>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();

        if let Some(a) = out.get_mut(a) {
            a.push(b);
        } else {
            out.insert(a, vec![b]);
        }

        if let Some(b) = out.get_mut(b) {
            b.push(a);
        } else {
            out.insert(b, vec![a]);
        }
    }

    for connections in out.values_mut() {
        connections.sort_unstable();
    }

    out
}

fn filter_connections3<'a>(
    connections: &'a HashMap<Computer, Vec<Computer>>,
    list: &[Computer<'a>],
) -> Vec<Computer<'a>> {
    list.iter()
        .cloned()
        .filter(|con1| {
            connections
                .get(con1)
                .unwrap()
                .iter()
                .any(|con2| list.contains(con2))
        })
        .collect()
}

fn connected3<'a>(
    connections: &'a HashMap<Computer, Vec<Computer>>,
    computer: &'a Computer,
    connections2: &[Computer<'a>],
) -> HashSet<[Computer<'a>; 3]> {
    let mut out: HashSet<[Computer<'a>; 3]> = HashSet::new();

    for (i, com2) in connections2.iter().enumerate() {
        for com3 in connections2.iter().skip(i + 1) {
            if !connections.get(com2).unwrap().contains(com3) {
                continue;
            }

            let mut list = [computer, *com2, *com3];
            list.sort_unstable();

            out.insert(list);
        }
    }

    out
}

fn solve(input: &str) -> usize {
    let connections = parse_input(input);

    let filtered: HashMap<Computer, Vec<Computer>> = connections
        .iter()
        .filter(|(computer, connections2)| computer.starts_with('t') && connections2.len() >= 2)
        .map(|(computer, connections2)| {
            (*computer, filter_connections3(&connections, connections2))
        })
        .filter(|(_, connections2)| connections2.len() >= 2)
        .collect();

    let connected3: HashSet<[Computer; 3]> = filtered
        .iter()
        .flat_map(|(computer, connections2)| connected3(&connections, computer, connections2))
        .collect();

    /* println!("{}", connected3.len());
    for x in &connected3 {
        println!("{},{},{}", x[0], x[1], x[2]);
    } */

    connected3.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_part1() {
        let input = "kh-tc
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
td-yn";
        let output = solve(input);
        assert_eq!(output, 7)
    }
}
