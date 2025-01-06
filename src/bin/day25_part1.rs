// cargo run  --bin day01_part1
// cargo test --bin day01_part1

fn main() {
    let input = include_str!("../././input/day01.txt");
    let output = solve(input);
    println!("Day01 part1: {output}");
}

fn solve(input: &str) -> u32 {
    let total: u32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let output = solve(input);
        assert_eq!(output, 3)
    }
}
