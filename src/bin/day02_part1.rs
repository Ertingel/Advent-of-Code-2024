// cargo run  --bin day02_part1
// cargo test --bin day02_part1

fn main() {
    let input = include_str!("../././input/day02.txt");
    let output = solve(input);
    println!("Day02 part1: {output}");
}

fn solve(input: &str) -> i32 {
    // Parsing input.
    let lists: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    println!("{:?}", lists);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let output = solve(input);
        assert_eq!(output, 2)
    }
}
