// cargo run  --bin day11_part2
// cargo test --bin day11_part2

use std::collections::HashMap;

fn main() {
    let input = include_str!("../././input/day11.txt");
    let output = solve(input, 75);
    println!("Day11 part2: {output}");
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split(' ').map(|num| num.parse().unwrap()).collect()
}

fn blink(memoize: &mut HashMap<(u64, u16), u64>, stone: u64, times: u16) -> u64 {
    if let Some(result) = memoize.get(&(stone, times)) {
        return *result;
    }

    if times == 0 {
        memoize.insert((stone, times), 1);
        return 1;
    }

    if stone == 0 {
        let result = blink(memoize, 1, times - 1);
        memoize.insert((stone, times), result);
        return result;
    }

    let str = stone.to_string();
    if str.len() % 2 == 0 {
        let result1 = blink(memoize, str[0..str.len() / 2].parse().unwrap(), times - 1);
        let result2 = blink(memoize, str[str.len() / 2..].parse().unwrap(), times - 1);

        let result = result1 + result2;
        memoize.insert((stone, times), result);
        return result;
    }

    let result = blink(memoize, stone * 2024, times - 1);
    memoize.insert((stone, times), result);
    result
}

fn solve(input: &str, times: u16) -> u64 {
    let stones = parse_input(input);

    let mut memoize: HashMap<(u64, u16), u64> = HashMap::new();

    stones
        .iter()
        .map(|stone| blink(&mut memoize, *stone, times))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part2_main_example1() {
        let input = "125 17";
        let output = solve(input, 25);
        assert_eq!(output, 55312)
    }

    #[test]
    fn day11_part2_main_example2() {
        let input = "125 17";
        let output = solve(input, 6);
        assert_eq!(output, 22)
    }

    #[test]
    fn day11_part2_example2() {
        let input = "0 1 10 99 999";
        let output = solve(input, 1);
        assert_eq!(output, 7)
    }
}
