// cargo run  --bin day10_part1
// cargo test --bin day10_part1

fn main() {
    let input = include_str!("../././input/day10.txt");
    let output = solve(input);
    println!("Day10 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let scores: i32 = 0;

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_main_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let output = solve(input);
        assert_eq!(output, 36)
    }

    #[test]
    fn day10_part1_example2() {
        let input = "0123
1234
8765
9876";
        let output = solve(input);
        assert_eq!(output, 1)
    }

    #[test]
    fn day10_part1_example3() {
        let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        let output = solve(input);
        assert_eq!(output, 2)
    }

    #[test]
    fn day10_part1_example4() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let output = solve(input);
        assert_eq!(output, 4)
    }

    #[test]
    fn day10_part1_example5() {
        let input = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        let output = solve(input);
        assert_eq!(output, 3)
    }
}
