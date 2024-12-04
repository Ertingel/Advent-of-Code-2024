// cargo run  --bin day03_part1
// cargo test --bin day03_part1

fn main() {
    let input = include_str!("../././input/day03.txt");
    let output = solve(input);
    println!("Day03 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let indices = input.match_indices("mul(");

    //let iter = input.chars().peekable();

    /*
    println!("{total}");
    // 2*4 + 5*5 + 11*8 + 8*5 = 161
     */

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let output = solve(input);
        assert_eq!(output, 161)
    }
}
