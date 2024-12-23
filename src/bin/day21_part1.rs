// cargo run  --bin day21_part1
// cargo test --bin day21_part1

fn main() {
    let input = include_str!("../././input/day21.txt");
    let output = solve(input);
    println!("Day21 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_part1() {
        let input = "029A
980A
179A
456A
379A";
        let output = solve(input);
        assert_eq!(output, 126384)
    }
}
