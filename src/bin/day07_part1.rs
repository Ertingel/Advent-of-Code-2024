// cargo run  --bin day07_part1
// cargo test --bin day07_part1

fn main() {
    let input = include_str!("../././input/day07.txt");
    let output = solve(input);
    println!("Day07 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    /*
    println!("{total}");
    // 2 + 1 + 0 + 1 + 2 + 5 = 11
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let output = solve(input);
        assert_eq!(output, 3749)
    }
}
