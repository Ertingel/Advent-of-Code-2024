// cargo run  --bin day22_part1
// cargo test --bin day22_part1

fn main() {
    let input = include_str!("../././input/day22.txt");
    let output = solve(input, 2000);
    println!("Day22 part1: {output}");
}

type Number = u64;

fn parse_input(input: &str) -> Vec<Number> {
    input
        .lines()
        .map(|line| line.parse::<Number>().unwrap())
        .collect()
}

fn mix(a: Number, b: Number) -> Number {
    a ^ b
}

fn prune(a: Number) -> Number {
    a % 16777216
}

fn next_number(number: Number) -> Number {
    let mut number = prune(mix(number * 64, number));
    number = prune(mix(number / 32, number));
    prune(mix(number * 2048, number))
}

fn number_at_itteration(number: Number, iterations: usize) -> Number {
    let mut number = number;

    for _ in 0..iterations {
        number = next_number(number);
    }

    number
}

fn solve(input: &str, iterations: usize) -> Number {
    let initial_number = parse_input(input);
    let final_number: Vec<Number> = initial_number
        .iter()
        .map(|number| number_at_itteration(*number, iterations))
        .collect();

    let total: Number = final_number.iter().sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_part1_main_example() {
        let input = "1
10
100
2024";
        let output = solve(input, 2000);
        assert_eq!(output, 37327623)
    }

    #[test]
    fn day22_part1_example2() {
        let input = "123";
        assert_eq!(
            solve(input, 1),
            15887950,
            "Result from iteration 1 does not match! "
        );
        assert_eq!(
            solve(input, 2),
            16495136,
            "Result from iteration 2 does not match! "
        );
        assert_eq!(
            solve(input, 3),
            527345,
            "Result from iteration 3 does not match! "
        );
        assert_eq!(
            solve(input, 4),
            704524,
            "Result from iteration 4 does not match! "
        );
        assert_eq!(
            solve(input, 5),
            1553684,
            "Result from iteration 5 does not match! "
        );
        assert_eq!(
            solve(input, 6),
            12683156,
            "Result from iteration 6 does not match! "
        );
        assert_eq!(
            solve(input, 7),
            11100544,
            "Result from iteration 7 does not match! "
        );
        assert_eq!(
            solve(input, 8),
            12249484,
            "Result from iteration 8 does not match! "
        );
        assert_eq!(
            solve(input, 9),
            7753432,
            "Result from iteration 9 does not match! "
        );
        assert_eq!(
            solve(input, 10),
            5908254,
            "Result from iteration 10 does not match! "
        );
    }

    #[test]
    fn day22_part1_test1() {
        assert_eq!(mix(42, 15), 37)
    }

    #[test]
    fn day22_part1_test2() {
        assert_eq!(prune(100000000), 16113920)
    }

    #[test]
    fn day22_part1_example3() {
        let input = "1";
        let output = solve(input, 2000);
        assert_eq!(output, 8685429)
    }

    #[test]
    fn day22_part1_example4() {
        let input = "10";
        let output = solve(input, 2000);
        assert_eq!(output, 4700978)
    }

    #[test]
    fn day22_part1_example5() {
        let input = "100";
        let output = solve(input, 2000);
        assert_eq!(output, 15273692)
    }

    #[test]
    fn day22_part1_example6() {
        let input = "2024";
        let output = solve(input, 2000);
        assert_eq!(output, 8667524)
    }
}
