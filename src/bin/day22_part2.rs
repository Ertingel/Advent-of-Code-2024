// cargo run  --bin day22_part2
// cargo test --bin day22_part2

fn main() {
    let input = include_str!("../././input/day22.txt");
    let output = solve(input, 2000);
    println!("Day22 part2: {output}");
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
    fn day22_part2_main_example() {
        let input = "1
10
100
2024";
        let output = solve(input, 2000);
        assert_eq!(output, 23)
    }
}
