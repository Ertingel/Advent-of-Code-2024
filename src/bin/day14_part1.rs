// cargo run  --bin day14_part1
// cargo test --bin day14_part1

use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day14.txt");
    let output = solve(input, (101, 103));
    println!("Day14 part1: {output}");
}

type Point = (i16, i16);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Robot {
    x: i16,
    y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl Robot {
    fn from_string(data: &str) -> Option<(&str, Self)> {
        //p=0,4 v=3,-3

        let (data, _) = parsing::string("p=")(data)?;
        let (data, x) = parsing::number::<i16>()(data)?;
        let (data, _) = parsing::char(',')(data)?;
        let (data, y) = parsing::number::<i16>()(data)?;
        let (data, _) = parsing::string(" v=")(data)?;
        let (data, velocity_x) = parsing::number::<i16>()(data)?;
        let (data, _) = parsing::char(',')(data)?;
        let (data, velocity_y) = parsing::number::<i16>()(data)?;

        Some((
            data,
            Robot {
                x,
                y,
                velocity_x,
                velocity_y,
            },
        ))
    }

    fn move_robot(&self, space: &Point, delta_time: &i16) -> Self {
        let (width, height) = *space;
        let mut clone = *self;

        clone.x = (clone.x + clone.velocity_x * delta_time).rem_euclid(width);
        clone.y = (clone.y + clone.velocity_y * delta_time).rem_euclid(height);

        clone
    }

    fn quadrant(&self, space: &Point) -> Option<usize> {
        let (width, height) = *space;
        let x_mid = width / 2;
        let y_mid = height / 2;

        if self.x == x_mid || self.y == y_mid {
            return None;
        }

        match (self.x >= x_mid, self.y >= y_mid) {
            (false, false) => Some(0),
            (true, false) => Some(1),
            (false, true) => Some(2),
            (true, true) => Some(3),
        }
    }
}
/*
fn print_robots(space: &Point, robots: &[Robot]) {
    let (width, height) = *space;
    let x_mid = width / 2;
    let y_mid = height / 2;

    for y in 0..height {
        for x in 0..width {
            let count = robots
                .iter()
                .filter(|robot| robot.x == x && robot.y == y)
                .count();
            if count > 0 {
                print!("{count}");
            } else {
                match (x == x_mid, y == y_mid) {
                    (false, false) => print!("."),
                    (true, false) => print!("|"),
                    (false, true) => print!("-"),
                    (true, true) => print!("+"),
                }
            }
        }
        println!();
    }

    let count = robots.iter().filter(|robot| robot.x < 0).count();
    println!("(x <  0     ) => {count}");

    let count = robots.iter().filter(|robot| robot.x >= width).count();
    println!("(x >= width ) => {count}");

    let count = robots.iter().filter(|robot| robot.y < 0).count();
    println!("(y <  0     ) => {count}");

    let count = robots.iter().filter(|robot| robot.y >= height).count();
    println!("(y >= height) => {count}");
}
 */
fn solve(input: &str, space: Point) -> i32 {
    let robots: Vec<Robot> = input
        .lines()
        .map(|line| Robot::from_string(line).unwrap().1)
        .collect();

    let moved_robots: Vec<Robot> = robots
        .iter()
        .map(|robot| robot.move_robot(&space, &100))
        .collect();

    /*
    print_robots(&space, &robots);
    println!();
    print_robots(&space, &moved_robots);
     */
    let mut quadrants: [i32; 4] = [0; 4];

    for robot in &moved_robots {
        if let Some(quad) = robot.quadrant(&space) {
            quadrants[quad] += 1;
        }
    }

    let safety_factor: i32 = quadrants.iter().product();

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
