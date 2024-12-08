// cargo run  --bin day08_part2
// cargo test --bin day08_part2

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../././input/day08.txt");
    let output = solve(input);
    println!("Day08 part2: {output}");
}

type Point = (i32, i32);

fn parse_input(input: &str) -> (Point, Vec<(Point, char)>) {
    let width: i32 = input.find('\n').unwrap() as i32;
    let height: i32 = input.matches('\n').count() as i32 + 1;

    let antennas: Vec<(Point, char)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<(Point, char)> {
            line.chars()
                .enumerate()
                .filter(|(_, frequency)| *frequency != '.')
                .map(|(x, frequency)| ((x as i32, y as i32), frequency))
                .collect()
        })
        .collect();

    ((width, height), antennas)
}

fn seperate_frequencies(input: &Vec<(Point, char)>) -> HashMap<char, HashSet<Point>> {
    let mut out: HashMap<char, HashSet<Point>> = HashMap::new();

    for (point, frequency) in input {
        let frequency_set = out.get_mut(frequency);
        if let Some(frequency_set) = frequency_set {
            frequency_set.insert(*point);
        } else {
            let mut frequency_set: HashSet<Point> = HashSet::new();
            frequency_set.insert(*point);
            out.insert(*frequency, frequency_set);
        }
    }

    out
}

fn is_in_bounds(dimensions: Point, point: Point) -> bool {
    0 <= point.0 && point.0 < dimensions.0 && 0 <= point.1 && point.1 < dimensions.1
}

fn get_nodes(dimensions: Point, antennas: &HashSet<Point>) -> HashSet<Point> {
    let combinations: Vec<(Point, Point)> = antennas
        .iter()
        .flat_map(|a| -> Vec<(Point, Point)> {
            antennas
                .iter()
                .map(|b| -> (Point, Point) { (*a, *b) })
                .filter(|(a, b)| a != b)
                .collect()
        })
        .collect();

    let count = dimensions.0.max(dimensions.1);

    let nodes: HashSet<Point> = combinations
        .iter()
        .flat_map(|((x1, y1), (x2, y2))| {
            let dx = x1 - x2;
            let dy = y1 - y2;

            (0..count).map(move |i| (x1 + dx * i, y1 + dy * i))
        })
        .filter(|node| is_in_bounds(dimensions, *node))
        .collect();

    nodes
}

fn solve(input: &str) -> i32 {
    let (dimensions, antennas) = parse_input(input);

    let frequencies = seperate_frequencies(&antennas);

    let nodes: HashMap<char, HashSet<Point>> = frequencies
        .iter()
        .map(|(key, list)| (*key, get_nodes(dimensions, list)))
        .collect();

    //let total: i32 = nodes.values().map(|list| list.len() as i32).sum();

    let total: i32 = nodes
        .values()
        .flatten()
        .cloned()
        .collect::<HashSet<Point>>()
        .len() as i32;

    /*
    println!("{:?}", antennas);
    println!("{:?}", frequencies);
    println!("{:?}", nodes);
    println!("{total}");
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part2_main_example() {
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
        assert_eq!(output, 34)
    }

    #[test]
    fn day08_part2_example2() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        let output = solve(input);
        assert_eq!(output, 9)
    }
}
