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

fn dos<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    |input: &str| parsing::take_untill(parsing::string("do()"))(input)
}

fn donts<'a>() -> impl Fn(&'a str) -> Option<(&'a str, &'a str)> {
    |input: &str| parsing::take_untill(parsing::string("don't()"))(input)
}

fn do_dont(input: &str) -> Vec<&str> {
    let mut remaining = input;
    let mut out: Vec<&str> = Vec::new();

    while !remaining.is_empty() {
        let res = donts()(remaining);

        if let Some((rem, take)) = res {
            remaining = rem;
            out.push(take);
        } else {
            break;
        }

        let res = dos()(remaining);
        if let Some((rem, _)) = res {
            remaining = rem;
        } else {
            break;
        }
    }

    out
}

fn solve(input: &str) -> i32 {
    let doos = do_dont(input);
    let mults: Vec<Vec<i32>> = doos
        .iter()
        .map(|input| parsing::find_all(mul())(input))
        .collect();

    let total: i32 = mults.iter().map(|x| x.iter().sum::<i32>()).sum();

    /*
    println!("{:?}", doos);
    println!("{:?}", mults);
    println!("{total}");
    // 2*4 + 8*5 = 48
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let output = solve(input);
        assert_eq!(output, 48)
    }
}
