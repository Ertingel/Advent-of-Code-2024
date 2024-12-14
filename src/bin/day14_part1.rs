// cargo run  --bin day14_part1
// cargo test --bin day14_part1

fn main() {
    let input = include_str!("../././input/day14.txt");
    let output = solve(input, (101, 103));
    println!("Day14 part1: {output}");
}

type Point = (i16, i16);

fn solve(input: &str, space: Point) -> i32 {
    let safety_factor: i32 = 0;

    safety_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let output = solve(input, (11, 7));
        assert_eq!(output, 12)
    }
}
