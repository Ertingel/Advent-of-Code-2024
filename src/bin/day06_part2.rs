// cargo run  --bin day06_part2
// cargo test --bin day06_part2

use std::collections::HashSet;

fn main() {
    let input = include_str!("../././input/day06.txt");
    let output = solve(input);
    println!("Day06 part2: {output}");
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Heading {
    Upp,
    Right,
    Down,
    Left,
    Oustide,
    Looping,
}

impl Heading {
    pub fn x(&self) -> i32 {
        match self {
            Heading::Upp => 0,
            Heading::Right => 1,
            Heading::Down => 0,
            Heading::Left => -1,
            Heading::Oustide => 0,
            Heading::Looping => 0,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Heading::Upp => -1,
            Heading::Right => 0,
            Heading::Down => 1,
            Heading::Left => 0,
            Heading::Oustide => 0,
            Heading::Looping => 0,
        }
    }

    pub fn rotate_clockwise(&self) -> Heading {
        match self {
            Heading::Upp => Heading::Right,
            Heading::Right => Heading::Down,
            Heading::Down => Heading::Left,
            Heading::Left => Heading::Upp,
            Heading::Oustide => Heading::Oustide,
            Heading::Looping => Heading::Looping,
        }
    }

    pub fn forward(&self, point: &Point) -> Point {
        (point.0 + self.x(), point.1 + self.y())
    }
}

fn step(
    dimensions: (usize, usize),
    obstructions: &HashSet<Point>,
    guard_position: Point,
    guard_heading: Heading,
) -> (Point, Heading) {
    let next_pos = guard_heading.forward(&guard_position);

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
        return (guard_position, guard_heading.rotate_clockwise());
    }

    (next_pos, guard_heading)
}

fn walk(
    dimensions: (usize, usize),
    obstructions: &HashSet<Point>,
    guard_position: Point,
    guard_heading: Heading,
) -> (HashSet<(Point, Heading)>, Heading) {
    let mut guard_position = guard_position;
    let mut guard_heading = guard_heading;
    let mut visited: HashSet<(Point, Heading)> = HashSet::new();

    while guard_heading != Heading::Oustide && guard_heading != Heading::Looping {
        visited.insert((guard_position, guard_heading.clone()));
        //print(dimensions, obstructions, guard_position, &visited);

        let res = step(dimensions, obstructions, guard_position, guard_heading);
        guard_position = res.0;
        guard_heading = res.1;

        if visited.contains(&(guard_position, guard_heading.clone())) {
            return (visited, Heading::Looping);
        }
    }

    (visited, guard_heading)
}

fn find_spots(
    dimensions: (usize, usize),
    obstructions: &HashSet<Point>,
    starting_guard_position: Point,
    starting_guard_heading: Heading,
) -> HashSet<Point> {
    let mut guard_position = starting_guard_position;
    let mut guard_heading = starting_guard_heading.clone();

    let mut spots: HashSet<Point> = HashSet::new();

    while guard_heading != Heading::Oustide && guard_heading != Heading::Looping {
        //print(dimensions, obstructions, guard_position, &visited);

        let test_obstruction = guard_heading.forward(&guard_position);

        if 0 <= test_obstruction.0
            && test_obstruction.0 < dimensions.0 as i32
            && 0 <= test_obstruction.1
            && test_obstruction.1 < dimensions.1 as i32
        {
            let mut obstructions_copy = obstructions.clone();
            obstructions_copy.insert(test_obstruction);
            let (_, result) = walk(
                dimensions,
                &obstructions_copy,
                starting_guard_position,
                starting_guard_heading.clone(),
            );

            if result == Heading::Looping {
                spots.insert(test_obstruction);
            }
        }

        let res = step(
            dimensions,
            obstructions,
            guard_position,
            guard_heading.clone(),
        );
        guard_position = res.0;
        guard_heading = res.1;
    }

    spots
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

    let spots = find_spots(dimensions, &obstructions, guard_position, Heading::Upp);

    /*
    println!("{total}");
    */

    spots.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part2() {
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
        assert_eq!(output, 6)
    }
}
