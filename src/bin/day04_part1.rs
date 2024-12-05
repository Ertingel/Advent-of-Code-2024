// cargo run  --bin day04_part1
// cargo test --bin day04_part1

use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day04.txt");
    let output = solve(input);
    println!("Day04 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    //println!("{total}");
    // 2*4 + 5*5 + 11*8 + 8*5 = 161

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let output = solve(input);
        assert_eq!(output, 18)
    }
}
