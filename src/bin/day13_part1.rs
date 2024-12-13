// cargo run  --bin day13_part1
// cargo test --bin day13_part1

fn main() {
    let input = include_str!("../././input/day13.txt");
    let output = solve(input);
    println!("Day13 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let output = solve(input);
        assert_eq!(output, 480)
    }
}
