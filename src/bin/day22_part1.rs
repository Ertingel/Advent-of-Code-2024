// cargo run  --bin day22_part1
// cargo test --bin day22_part1

fn main() {
    let input = include_str!("../././input/day22.txt");
    let output = solve(input, 2000);
    println!("Day22 part1: {output}");
}

fn solve(input: &str, iterations: usize) -> u32 {
    let total: u32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_part1_main_example() {
        let input = "1
10
100
2024";
        let output = solve(input, 2000);
        assert_eq!(output, 37327623)
    }

    #[test]
    fn day22_part1_example2() {
        let input = "123";
        assert_eq!(solve(input, 1), 15887950);
        assert_eq!(solve(input, 2), 16495136);
        assert_eq!(solve(input, 3), 527345);
        assert_eq!(solve(input, 4), 704524);
        assert_eq!(solve(input, 5), 1553684);
        assert_eq!(solve(input, 6), 12683156);
        assert_eq!(solve(input, 7), 11100544);
        assert_eq!(solve(input, 8), 12249484);
        assert_eq!(solve(input, 9), 7753432);
        assert_eq!(solve(input, 10), 5908254);
    }

    #[test]
    fn day22_part1_example3() {
        let input = "1";
        let output = solve(input, 2000);
        assert_eq!(output, 8685429)
    }

    #[test]
    fn day22_part1_example4() {
        let input = "10";
        let output = solve(input, 2000);
        assert_eq!(output, 4700978)
    }

    #[test]
    fn day22_part1_example5() {
        let input = "100";
        let output = solve(input, 2000);
        assert_eq!(output, 15273692)
    }

    #[test]
    fn day22_part1_example6() {
        let input = "2024";
        let output = solve(input, 2000);
        assert_eq!(output, 8667524)
    }
}
