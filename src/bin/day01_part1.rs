fn main() {
    let input = include_str!("../././input/day01.txt");
    let output = solve(input);
    print!("day01 part1: {output}");
}

fn solve(input: &str) -> String {
    todo!();
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1_example1() {
        let input = "";
        let output = solve(input);
        assert_eq!(output, "".to_string())
    }

    #[test]
    fn day01_part1_example2() {
        let input = "";
        let output = solve(input);
        assert_eq!(output, "".to_string())
    }
}
