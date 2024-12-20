// cargo run  --bin day01_part1
// cargo test --bin day01_part1

fn main() {
    let input = include_str!("../././input/day01.txt");
    let output = solve(input);
    println!("Day01 part1: {output}");
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

    // Sorting lists.
    let mut lists = lists;
    lists.iter_mut().for_each(|list| list.sort());

    // Transposing lists.
    let cols = lists[0].len();
    let lists: Vec<Vec<i32>> = (0..cols)
        .map(|col| lists.iter().map(|row| row[col]).collect())
        .collect();

    // Geting deltas.
    let list: Vec<i32> = lists.iter().map(|row| (row[0] - row[1]).abs()).collect();

    // Getting total.
    let total: i32 = list.iter().sum();

    /*
    println!("{:?}", lists);
    println!("{:?}", list);
    println!("{total}");
    // 2 + 1 + 0 + 1 + 2 + 5 = 11
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
        assert_eq!(output, 11)
    }
}
