// cargo run  --bin day11_part1
// cargo test --bin day11_part1

fn main() {
    let input = include_str!("../././input/day11.txt");
    let output = solve(input, 25);
    println!("Day11 part1: {output}");
}

fn solve(input: &str, times: usize) -> i32 {
    let total: i32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_main_example1() {
        let input = "125 17";
        let output = solve(input, 25);
        assert_eq!(output, 55312)
    }

    #[test]
    fn day11_part1_main_example2() {
        let input = "125 17";
        let output = solve(input, 6);
        assert_eq!(output, 22)
    }

    #[test]
    fn day11_part1_example2() {
        let input = "0 1 10 99 999";
        let output = solve(input, 1);
        assert_eq!(output, 7)
    }
}
