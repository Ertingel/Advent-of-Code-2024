// cargo run  --bin day24_part1
// cargo test --bin day24_part1

use std::collections::HashMap;

fn main() {
    let input = include_str!("../././input/day24.txt");
    let output = solve(input);
    println!("Day24 part1: {output}");
}

type Link<'a> = &'a str;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl GateType {
    fn from_str(input: &str) -> Self {
        match input {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => panic!(),
        }
    }

    fn execute(&self, x: bool, y: bool) -> bool {
        match self {
            GateType::And => x && y,
            GateType::Or => x || y,
            GateType::Xor => x ^ y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Gate<'a> {
    gate_type: GateType,
    x: Link<'a>,
    y: Link<'a>,
    out: Link<'a>,
}

impl<'a> Gate<'a> {
    fn new(gate_type: GateType, x: Link<'a>, y: Link<'a>, out: Link<'a>) -> Self {
        Gate {
            gate_type,
            x,
            y,
            out,
        }
    }

    fn execute(&self, links: &mut HashMap<Link<'a>, Option<bool>>) -> Option<bool> {
        let out = self
            .gate_type
            .execute((*links.get(self.x)?)?, (*links.get(self.y)?)?);

        let out = Some(out);
        links.insert(self.out, out);

        out
    }
}

fn parse_input(input: &str) -> (HashMap<Link, Option<bool>>, Vec<Gate>) {
    let (initials, gates) = input.split_once("\n\n").unwrap();

    let mut links: HashMap<Link, Option<bool>> = initials
        .lines()
        .map(|line| {
            let (link, state) = line.split_once(": ").unwrap();

            (link, Some(state == "1"))
        })
        .collect();

    let gates: Vec<Gate> = gates
        .lines()
        .map(|line| {
            let mut split = line.split(' ');

            let x = split.next().unwrap();
            let gate_type = GateType::from_str(split.next().unwrap());
            let y = split.next().unwrap();
            split.next().unwrap();
            let out = split.next().unwrap();

            if !links.contains_key(out) {
                links.insert(out, None);
            }

            Gate::new(gate_type, x, y, out)
        })
        .collect();

    (links, gates)
}

fn execute<'a>(
    links: &HashMap<Link<'a>, Option<bool>>,
    gates: &[Gate<'a>],
) -> HashMap<Link<'a>, Option<bool>> {
    let mut links = links.clone();

    let mut updating = true;

    while updating {
        updating = false;

        for gate in gates {
            let old_state = *links.get(gate.out).unwrap();
            if old_state != gate.execute(&mut links) {
                updating = true;
            }
        }
    }

    links
}

fn get_z_value(links: &HashMap<Link, Option<bool>>) -> u64 {
    //println!("{:?}", links);
    //print_links(links);

    let mut list: Vec<(u16, bool)> = links
        .iter()
        .filter(|(name, value)| name.starts_with('z') && value.is_some())
        .map(|(name, value)| (name[1..].parse::<u16>().unwrap(), value.unwrap()))
        .collect();

    list.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    //println!("{:?}", list);

    list.iter()
        .map(|(name, value)| (*value as u64) << name)
        .reduce(|a, b| a | b)
        .unwrap()
}

/* fn print_links(links: &HashMap<Link, Option<bool>>) {
    let mut links: Vec<(Link, Option<bool>)> = links.iter().map(|(k, v)| (*k, *v)).collect();
    links.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (name, value) in links {
        if let Some(value) = value {
            if value {
                println!("{}: 1", name);
            } else {
                println!("{}: 0", name);
            }
        } else {
            println!("{}: _", name);
        }
    }
} */

fn solve(input: &str) -> u64 {
    let (links, gates) = parse_input(input);
    let result = execute(&links, &gates);
    let z: u64 = get_z_value(&result);

    z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24_part1_main_example() {
        let input = "x00: 1
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
tnw OR pbm -> gnj";
        let output = solve(input);
        assert_eq!(output, 2024)
    }

    #[test]
    fn day24_part1_example2() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let output = solve(input);
        assert_eq!(output, 0b_100)
    }
}
