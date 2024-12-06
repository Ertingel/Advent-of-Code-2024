// cargo run  --bin day06_part1
// cargo test --bin day06_part1

fn main() {
    let input = include_str!("../././input/day06.txt");
    let output = solve(input);
    println!("Day06 part1: {output}");
}

fn solve(input: &str) -> i32 {
    // Getting total.
    let total: i32 = 0;

    /*
    println!("{total}");
    */
    /*
    ....#.....
    ....XXXXX#
    ....X...X.
    ..#.X...X.
    ..XXXXX#X.
    ..X.X.X.X.
    .#XXXXXXX.
    .XXXXXXX#.
    #XXXXXXX..
    ......#X..

    41 X's total.
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let output = solve(input);
        assert_eq!(output, 41)
    }
}
