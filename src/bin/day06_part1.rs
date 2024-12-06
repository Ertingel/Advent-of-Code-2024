// cargo run  --bin day06_part1
// cargo test --bin day06_part1

use std::collections::HashSet;

fn main() {
    let input = include_str!("../././input/day06.txt");
    let output = solve(input);
    println!("Day06 part1: {output}");
}

type Point = (i32, i32);

fn parse_map(input: &str) -> ((usize, usize), HashSet<Point>, Point) {
    let mut guard_position: Point = (0, 0);
    let mut obstructions: HashSet<Point> = HashSet::new();

    let width: usize = input.find('\n').unwrap();
    let height: usize = input.matches('\n').count() + 1;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let point = (x as i32, y as i32);
            match char {
                '#' => {
                    obstructions.insert(point);
                }
                '^' => {
                    guard_position = point;
                }
                _ => {}
            }
        }
    }

    ((width, height), obstructions, guard_position)
}

#[derive(PartialEq)]
enum Heading {
    Upp,
    Right,
    Down,
    Left,
    Oustide,
}

impl Heading {
    pub fn x(&self) -> i32 {
        match self {
            Heading::Upp => 0,
            Heading::Right => 1,
            Heading::Down => 0,
            Heading::Left => -1,
            Heading::Oustide => 0,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Heading::Upp => -1,
            Heading::Right => 0,
            Heading::Down => 1,
            Heading::Left => 0,
            Heading::Oustide => 0,
        }
    }

    pub fn rotate_clockwise(&self) -> Heading {
        match self {
            Heading::Upp => Heading::Right,
            Heading::Right => Heading::Down,
            Heading::Down => Heading::Left,
            Heading::Left => Heading::Upp,
            Heading::Oustide => Heading::Oustide,
        }
    }
}

fn step(
    dimensions: (usize, usize),
    obstructions: &HashSet<Point>,
    guard_position: Point,
    guard_heading: Heading,
) -> (Point, Heading) {
    let (guard_position_x, guard_position_y) = guard_position;
    let next_pos = (
        guard_position_x + guard_heading.x(),
        guard_position_y + guard_heading.y(),
    );

    // Check if otside of bounds.
    if 0 > next_pos.0
        || next_pos.0 >= dimensions.0 as i32
        || 0 > next_pos.1
        || next_pos.1 >= dimensions.1 as i32
    {
        return (next_pos, Heading::Oustide);
    }

    // Check if colliding.
    if obstructions.contains(&next_pos) {
        return step(
            dimensions,
            obstructions,
            guard_position,
            guard_heading.rotate_clockwise(),
        );
    }

    (next_pos, guard_heading)
}

fn walk(
    dimensions: (usize, usize),
    obstructions: &HashSet<Point>,
    guard_position: Point,
    guard_heading: Heading,
) -> HashSet<Point> {
    let mut guard_position = guard_position;
    let mut guard_heading = guard_heading;
    let mut visited: HashSet<Point> = HashSet::new();

    while guard_heading != Heading::Oustide {
        visited.insert(guard_position);
        //print(dimensions, obstructions, guard_position, &visited);

        let res = step(dimensions, obstructions, guard_position, guard_heading);
        guard_position = res.0;
        guard_heading = res.1;
    }

    visited
}

/*
fn print(
    dimensions: (usize, usize),
    obstructions: &HashSet<Point>,
    guard_position: Point,
    visited: &HashSet<Point>,
) {
    let (width, height) = dimensions;

    for y in 0..height {
        for x in 0..width {
            let point = (x as i32, y as i32);
            if x as i32 == guard_position.0 && y as i32 == guard_position.1 {
                print!("^");
            } else if obstructions.contains(&point) {
                print!("#");
            } else if visited.contains(&point) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
*/

fn solve(input: &str) -> i32 {
    let (dimensions, obstructions, guard_position) = parse_map(input);

    let visited = walk(dimensions, &obstructions, guard_position, Heading::Upp);

    //print(dimensions, &obstructions, guard_position, &visited);

    /*
    println!("{total}");
    */
    /*
    ....#.....
    ....XXXXX#
    ....X...X.
    ..#.X...X.
    ..XXXXX#X.
    ..X.X.X.X.
    .#XXXXXXX.
    .XXXXXXX#.
    #XXXXXXX..
    ......#X..

    41 X's total.
    */

    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let output = solve(input);
        assert_eq!(output, 41)
    }
}
