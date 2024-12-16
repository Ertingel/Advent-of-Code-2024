// cargo run  --bin day16_part1
// cargo test --bin day16_part1

fn main() {
    let input = include_str!("../././input/day16.txt");
    let output = solve(input);
    println!("Day16 part1: {output}");
}

fn solve(input: &str) -> u32 {
    let score: u32 = 0;

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_main_example() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let output = solve(input);
        assert_eq!(output, 11048)
    }

    #[test]
    fn day16_part1_example2() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let output = solve(input);
        assert_eq!(output, 7036)
    }
}
