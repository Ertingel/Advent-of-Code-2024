// cargo run  --bin day23_part1
// cargo test --bin day23_part1

fn main() {
    let input = include_str!("../././input/day23.txt");
    let output = solve(input);
    println!("Day23 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    total
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
