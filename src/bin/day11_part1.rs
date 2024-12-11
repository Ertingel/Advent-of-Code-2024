// cargo run  --bin day11_part1
// cargo test --bin day11_part1

fn main() {
    let input = include_str!("../././input/day11.txt");
    let output = solve(input, 25);
    println!("Day11 part1: {output}");
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split(' ').map(|num| num.parse().unwrap()).collect()
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| -> Vec<u64> {
            if *stone == 0 {
                return vec![1];
            }

            let str = stone.to_string();
            if str.len() % 2 == 0 {
                return vec![
                    str[0..str.len() / 2].parse().unwrap(),
                    str[str.len() / 2..].parse().unwrap(),
                ];
            }

            vec![stone * 2024]
        })
        .collect()
}

fn solve(input: &str, times: usize) -> usize {
    let mut stones = parse_input(input);

    for _ in 0..times {
        stones = blink(&stones);
    }

    stones.len()
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
