// cargo run  --bin day19_part2
// cargo test --bin day19_part2

use std::collections::HashMap;

use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day19.txt");
    let output = solve(input);
    println!("Day19 part2: {output}");
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = towels.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    (towels, designs)
}

fn solvable<'a>(
    memoize: &mut HashMap<&'a str, u64>,
    towels: &Vec<&'a str>,
    design: &'a str,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(result) = memoize.get(design) {
        return *result;
    }

    let result = towels
        .iter()
        .map(|towel| {
            if let Some((remaining, _)) = parsing::string(towel)(design) {
                solvable(memoize, towels, remaining)
            } else {
                0
            }
        })
        .sum();

    memoize.insert(design, result);

    result
}

fn solve(input: &str) -> u64 {
    let (towels, designs) = parse_input(input);

    let mut memoize: HashMap<&str, u64> = HashMap::new();

    let total: u64 = designs
        .iter()
        .map(|design| solvable(&mut memoize, &towels, design))
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_part2() {
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
        assert_eq!(output, 16)
    }
}
