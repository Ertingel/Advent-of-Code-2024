// cargo run  --bin day19_part1
// cargo test --bin day19_part1

fn main() {
    let input = include_str!("../././input/day19.txt");
    let output = solve(input);
    println!("Day19 part1: {output}");
}

fn solve(input: &str) -> usize {
    let total: usize = 0;

    /*
    brwrr can be made with a br towel, then a wr towel, and then finally an r towel.
    bggr can be made with a b towel, two g towels, and then an r towel.
    gbbr can be made with a gb towel and then a br towel.
    rrbgbr can be made with r, rb, g, and br.
    ubwu is impossible.
    bwurrg can be made with bwu, r, r, and g.
    brgr can be made with br, g, and r.
    bbrgwb is impossible.
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_part1() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let output = solve(input);
        assert_eq!(output, 6)
    }
}
