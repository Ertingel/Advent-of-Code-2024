// cargo run  --bin day03_part2
// cargo test --bin day03_part2

use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day03.txt");
    let output = solve(input);
    println!("Day03 part2: {output}");
}

fn mul<'a>() -> impl Fn(&'a str) -> Option<(&'a str, i32)> {
    |input: &str| {
        let remaining = input;
        let (remaining, _) = parsing::string("mul(")(remaining)?;

        let (remaining, a) = parsing::continuous(1, 3, parsing::numeric())(remaining)?;
        let a = a.parse::<i32>().ok()?;

        let (remaining, _) = parsing::char(',')(remaining)?;

        let (remaining, b) = parsing::continuous(1, 3, parsing::numeric())(remaining)?;
        let b = b.parse::<i32>().ok()?;

        let (remaining, _) = parsing::char(')')(remaining)?;

        Some((remaining, a * b))
    }
}

fn solve(input: &str) -> i32 {
    let mults = parsing::find_all(mul())(input);
    let total: i32 = mults.iter().sum();

    println!("{total}");
    // 2*4 + 5*5 + 11*8 + 8*5 = 161

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part2() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let output = solve(input);
        assert_eq!(output, 48)
    }
}
