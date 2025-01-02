// cargo run  --bin day22_part2
// cargo test --bin day22_part2

use std::collections::HashMap;

fn main() {
    let input = include_str!("../././input/day22.txt");
    let output = solve(input, 2000);
    println!("Day22 part2: {output}");
}

type Number = u64;
type Change = i8;

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

fn numbers_list(number: Number, iterations: usize) -> Vec<Number> {
    let mut number = number;
    let mut out: Vec<Number> = vec![number];

    for _ in 0..iterations {
        number = next_number(number);
        out.push(number);
    }

    out
}

fn solve(input: &str, iterations: usize) -> Number {
    let initial_number = parse_input(input);

    let numbers_list: Vec<Vec<Number>> = initial_number
        .iter()
        .cloned()
        .map(|num| numbers_list(num, iterations))
        .collect();

    let changes: Vec<Vec<Change>> = numbers_list
        .iter()
        .map(|list| {
            list.windows(2)
                .map(|a| (a[1] % 10) as Change - (a[0] % 10) as Change)
                .collect()
        })
        .collect();

    let paterns: HashMap<[Change; 4], Number> = numbers_list
        .iter()
        .zip(&changes)
        .map(|(numbers, changes)| {
            let mut out: HashMap<[Change; 4], Number> = HashMap::new();

            numbers[4..]
                .iter()
                .zip(changes.windows(4))
                .for_each(|(quantity, changes)| {
                    if !out.contains_key(changes) {
                        let changes = [changes[0], changes[1], changes[2], changes[3]];
                        out.insert(changes, quantity % 10);
                    }
                });

            out
        })
        .fold(HashMap::new(), |mut a, b| {
            for (key, val) in b {
                if let Some(val2) = a.get(&key) {
                    a.insert(key, val + val2);
                } else {
                    a.insert(key, val);
                }
            }

            a
        });

    let max: Number = *paterns.values().max().unwrap();

    /*
    paterns
        .iter()
        .filter(|(_, value)| **value == max)
        .for_each(|(key, value)| {
            println!("{:?} {}", key, value);
        });

    println!("{:?}", paterns.get(&[-2, 1, -1, 3]));
    */

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_part2_main_example() {
        let input = "1
2
3
2024";
        let output = solve(input, 2000);
        assert_eq!(output, 23)
    }

    #[test]
    fn day22_part2_example2() {
        let input = "123";
        let output = solve(input, 10);
        assert_eq!(output, 6)
    }
}
