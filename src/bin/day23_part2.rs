// cargo run  --bin day23_part2
// cargo test --bin day23_part2

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../././input/day23.txt");
    let output = solve(input);
    println!("Day23 part2: {output}");
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

fn combinate<'a>(
    selection: &HashSet<Computer<'a>>,
    connections: &[(Computer<'a>, HashSet<Computer<'a>>)],
) -> HashSet<Computer<'a>> {
    let mut largest = selection.clone();

    for (i, (com, con)) in connections.iter().enumerate() {
        if selection.contains(com) || !con.is_superset(selection) {
            continue;
        }

        let mut selection2 = selection.clone();
        selection2.insert(com);

        let res = combinate(&selection2, &connections[i + 1..]);

        if res.len() > largest.len() {
            largest = res;
        }
    }

    largest
}

fn filter_connections<'a>(
    connections: &'a HashMap<Computer, Vec<Computer>>,
    list: &[Computer<'a>],
) -> Vec<Computer<'a>> {
    let set1: HashSet<Computer> = list.iter().cloned().collect();

    let sets: Vec<(Computer, HashSet<Computer>)> = list
        .iter()
        .map(|con| {
            let mut cons = connections
                .get(con)
                .unwrap()
                .iter()
                .filter(|e| set1.contains(*e))
                .cloned()
                .collect::<HashSet<Computer>>();
            cons.insert(con);

            (*con, cons)
        })
        .filter(|(_, cons)| !cons.is_empty())
        .collect();

    combinate(&HashSet::new(), &sets).iter().cloned().collect()
}

fn solve(input: &str) -> String {
    let connections = parse_input(input);

    let filtered: HashMap<Computer, Vec<Computer>> = connections
        .iter()
        .filter(|(computer, _)| computer.starts_with('t'))
        .map(|(computer, connections2)| (*computer, filter_connections(&connections, connections2)))
        .collect();

    let longest = filtered
        .values()
        .map(|connections2| connections2.len())
        .max()
        .unwrap();

    let (computer, connections2) = filtered
        .iter()
        .find(|(_, connections2)| connections2.len() == longest)
        .unwrap();

    let mut out = connections2.clone();
    out.push(computer);
    out.sort_unstable();

    out.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_part2() {
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
        assert_eq!(output, "co,de,ka,ta")
    }
}
