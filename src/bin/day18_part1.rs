// cargo run  --bin day18_part1
// cargo test --bin day18_part1

fn main() {
    let input = include_str!("../././input/day18.txt");
    let output = solve(input, 70, 1024);
    println!("Day18 part1: {output}");
}

fn solve(input: &str, space: usize, num_of_bytes: usize) -> i32 {
    let steps: i32 = 0;

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_part1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let output = solve(input, 6, 12);
        assert_eq!(output, 12)
    }
}
