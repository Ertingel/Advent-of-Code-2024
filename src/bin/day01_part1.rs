extern crate advent_of_code;
use advent_of_code::reader;

fn main() {
    let input = include_str!("../././input/day01.txt");
    let output = solve(input);
    print!("day01 part1: {output}");
}

fn solve(input: &str) -> String {
    let r = Reader::new(input);
    todo!();
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let output = solve(input);
        assert_eq!(output, "".to_string())
    }
}
