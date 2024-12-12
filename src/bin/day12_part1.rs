// cargo run  --bin day12_part1
// cargo test --bin day12_part1

fn main() {
    let input = include_str!("../././input/day12.txt");
    let output = solve(input);
    println!("Day12 part1: {output}");
}

fn solve(input: &str) -> i32 {
    let total: i32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_main_example() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let output = solve(input);
        assert_eq!(output, 1930)
    }

    #[test]
    fn day12_part1_example2() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let output = solve(input);
        assert_eq!(output, 140)
    }

    #[test]
    fn day12_part1_example3() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let output = solve(input);
        assert_eq!(output, 772)
    }
}
