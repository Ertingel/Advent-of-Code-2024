// cargo run  --bin day02_part2
// cargo test --bin day02_part2

fn main() {
    let input = include_str!("../././input/day02.txt");
    let output = solve(input);
    println!("Day02 part2: {output}");
}

fn copy_exclude(list: &[i32], index: i32) -> Vec<i32> {
    let mut clone = list.to_vec();
    if index >= 0 {
        clone.remove(index as usize);
    }
    clone
}

fn solve(input: &str) -> i32 {
    // Parsing input.
    let lists: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // Checking safe reports.
    let min_delta: i32 = 1;
    let max_delta: i32 = 3;
    let list: Vec<bool> = lists
        .iter()
        .map(|report| {
            (-1..(report.len() as i32))
                .map(|index| copy_exclude(report, index))
                .any(|report| {
                    let increasing = report
                        .windows(2)
                        .map(|window| window[1] - window[0])
                        .all(|delta| min_delta <= delta && delta <= max_delta);

                    let decreasing = report
                        .windows(2)
                        .map(|window| window[0] - window[1])
                        .all(|delta| min_delta <= delta && delta <= max_delta);

                    increasing || decreasing
                })
        })
        .collect();

    // Getting total.
    let total: i32 = list.iter().filter(|safe| **safe).count() as i32;

    /*
    println!("{:?}", lists);
    println!("{:?}", list);
    println!("{total}");
    // Safe Unsafe Unsafe Safe Safe Safe = 2
     */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let output = solve(input);
        assert_eq!(output, 4)
    }
}
