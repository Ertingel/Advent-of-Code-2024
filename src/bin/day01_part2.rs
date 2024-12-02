// cargo run  --bin day01_part2
// cargo test --bin day01_part2

extern crate advent_of_code;

fn main() {
    let input = include_str!("../././input/day01.txt");
    let output = solve(input);
    println!("day01 part1: {output}");
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

    // Transposing lists.
    let cols = lists[0].len();
    let lists: Vec<Vec<i32>> = (0..cols)
        .map(|col| lists.iter().map(|row| row[col]).collect())
        .collect();

    // Geting similarity scores.
    let list: Vec<i32> = lists[0]
        .iter()
        .map(|num| {
            let count = lists[1].iter().filter(|x| *x == num).count() as i32;
            num * count
        })
        .collect();

    // Getting total.
    let total: i32 = list.iter().sum();

    /*
    println!("{:?}", lists);
    println!("{:?}", list);
    println!("{total}");
    // 9 + 4 + 0 + 0 + 9 + 9 = 31
     */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let output = solve(input);
        assert_eq!(output, 31)
    }
}
