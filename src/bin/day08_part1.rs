// cargo run  --bin day08_part1
// cargo test --bin day08_part1

fn main() {
    let input = include_str!("../././input/day08.txt");
    let output = solve(input);
    println!("Day08 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    /*
    println!("{total}");
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part1_main_example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let output = solve(input);
        assert_eq!(output, 14)
    }

    #[test]
    fn day08_part1_example2() {
        let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";
        let output = solve(input);
        assert_eq!(output, 2)
    }

    #[test]
    fn day08_part1_example3() {
        let input = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";
        let output = solve(input);
        assert_eq!(output, 4)
    }
}
