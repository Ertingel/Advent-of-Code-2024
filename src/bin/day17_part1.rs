// cargo run  --bin day17_part1
// cargo test --bin day17_part1

fn main() {
    let input = include_str!("../././input/day17.txt");
    let output = solve(input);
    println!("Day17 part1: {output}");
}

fn solve(input: &str) -> String {
    let output: String = "".to_owned();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_part1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let output = solve(input);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0")
    }
}
