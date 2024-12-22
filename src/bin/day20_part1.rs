// cargo run  --bin day20_part1
// cargo test --bin day20_part1

fn main() {
    let input = include_str!("../././input/day20.txt");
    let output = solve(input);
    println!("Day20 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_part1() {
        let input = "###############
#...#...12....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let output = solve(input);
        assert_eq!(output, 40)
    }
}
