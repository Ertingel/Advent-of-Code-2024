// cargo run  --bin day05_part1
// cargo test --bin day05_part1

fn main() {
    let input = include_str!("../././input/day05.txt");
    let output = solve(input);
    println!("Day05 part2: {output}");
}

fn solve(input: &str) -> i32 {
    // Getting total.
    let total: i32 = 0;

    //println!("{total}");

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let output = solve(input);
        assert_eq!(output, 143)
    }
}
