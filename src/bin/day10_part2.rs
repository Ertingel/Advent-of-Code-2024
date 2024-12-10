// cargo run  --bin day10_part2
// cargo test --bin day10_part2

use std::collections::HashSet;

fn main() {
    let input = include_str!("../././input/day10.txt");
    let output = solve(input);
    println!("Day10 part2: {output}");
}

type Point = (i32, i32);
type Map = Vec<Vec<u8>>;

fn get_dimensions(map: &Map) -> Point {
    let width: i32 = map[0].len() as i32;
    let height: i32 = map.len() as i32;
    (width, height)
}

fn is_in_bounds(dimensions: Point, point: Point) -> bool {
    0 <= point.0 && point.0 < dimensions.0 && 0 <= point.1 && point.1 < dimensions.1
}

fn parse_input(input: &str) -> (Point, Map) {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap_or(255) as u8)
                .collect()
        })
        .collect();

    let dimensions = get_dimensions(&map);

    (dimensions, map)
}

fn find_trailheads(map: &Map) -> HashSet<Point> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| -> HashSet<Point> {
            row.iter()
                .enumerate()
                .filter_map(|(x, height)| {
                    if *height == 0 {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

fn walk_trailhead(dimensions: Point, map: &Map, point: Point) -> u32 {
    let (x, y) = point;
    let height = map[y as usize][x as usize];

    if height == 9 {
        return 1;
    }

    let directions = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)];

    directions
        .iter()
        .map(|point2| {
            let (x2, y2) = point2;

            if !is_in_bounds(dimensions, *point2) {
                return 0;
            }

            let height2 = map[*y2 as usize][*x2 as usize];

            if height2 != height + 1 {
                return 0;
            }

            walk_trailhead(dimensions, map, *point2)
        })
        .sum()
}

fn get_scores(dimensions: Point, map: &Map) -> u32 {
    let trailheads = find_trailheads(map);

    trailheads
        .iter()
        .map(|trailhead| walk_trailhead(dimensions, map, *trailhead))
        .sum()
}

fn solve(input: &str) -> u32 {
    let (dimensions, map) = parse_input(input);

    let scores: u32 = get_scores(dimensions, &map);

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part2_main_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let output = solve(input);
        assert_eq!(output, 81)
    }

    #[test]
    fn day10_part2_example2() {
        let input = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
        let output = solve(input);
        assert_eq!(output, 3)
    }

    #[test]
    fn day10_part2_example3() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let output = solve(input);
        assert_eq!(output, 13)
    }

    #[test]
    fn day10_part2_example4() {
        let input = "012345
123456
234567
345678
4.6789
56789.";
        let output = solve(input);
        assert_eq!(output, 227)
    }
}
