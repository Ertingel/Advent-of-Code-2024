// cargo run  --bin day21_part1
// cargo test --bin day21_part1

use core::fmt;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../././input/day21.txt");
    let output = solve(input);
    println!("Day21 part1: {output}");
}

type Point = (i8, i8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Keypad {
    Seven,
    Eight,
    Nine,

    Four,
    Five,
    Six,

    One,
    Two,
    Three,

    Zero,
    A,
}

impl Keypad {
    fn from_char(char: char) -> Self {
        match char {
            '7' => Keypad::Seven,
            '8' => Keypad::Eight,
            '9' => Keypad::Nine,

            '4' => Keypad::Four,
            '5' => Keypad::Five,
            '6' => Keypad::Six,

            '1' => Keypad::One,
            '2' => Keypad::Two,
            '3' => Keypad::Three,

            '0' => Keypad::Zero,
            'A' => Keypad::A,
            _ => panic!(),
        }
    }

    fn get_point(&self) -> Point {
        match self {
            Keypad::Seven => (0, 0),
            Keypad::Eight => (1, 0),
            Keypad::Nine => (2, 0),

            Keypad::Four => (0, 1),
            Keypad::Five => (1, 1),
            Keypad::Six => (2, 1),

            Keypad::One => (0, 2),
            Keypad::Two => (1, 2),
            Keypad::Three => (2, 2),

            Keypad::Zero => (1, 3),
            Keypad::A => (2, 3),
        }
    }

    fn types() -> [Keypad; 11] {
        [
            Keypad::Seven,
            Keypad::Eight,
            Keypad::Nine,
            Keypad::Four,
            Keypad::Five,
            Keypad::Six,
            Keypad::One,
            Keypad::Two,
            Keypad::Three,
            Keypad::Zero,
            Keypad::A,
        ]
    }
}

impl fmt::Display for Keypad {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keypad::Seven => fmt.write_str("7"),
            Keypad::Eight => fmt.write_str("8"),
            Keypad::Nine => fmt.write_str("9"),

            Keypad::Four => fmt.write_str("4"),
            Keypad::Five => fmt.write_str("5"),
            Keypad::Six => fmt.write_str("6"),

            Keypad::One => fmt.write_str("1"),
            Keypad::Two => fmt.write_str("2"),
            Keypad::Three => fmt.write_str("3"),

            Keypad::Zero => fmt.write_str("0"),
            Keypad::A => fmt.write_str("A"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Upp,
    A,

    Left,
    Down,
    Right,
}

impl Direction {
    fn from_char(char: char) -> Self {
        match char {
            '^' => Direction::Upp,
            'A' => Direction::A,

            '<' => Direction::Left,
            'v' => Direction::Down,
            '>' => Direction::Right,
            _ => panic!(),
        }
    }

    fn get_point(&self) -> Point {
        match self {
            Direction::Upp => (1, 0),
            Direction::A => (2, 0),

            Direction::Left => (0, 1),
            Direction::Down => (1, 1),
            Direction::Right => (2, 1),
        }
    }

    fn types() -> [Direction; 5] {
        [
            Direction::Upp,
            Direction::A,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Upp => fmt.write_str("^"),
            Direction::A => fmt.write_str("A"),

            Direction::Left => fmt.write_str("<"),
            Direction::Down => fmt.write_str("v"),
            Direction::Right => fmt.write_str(">"),
        }
    }
}

fn parse_keypad(code: &str) -> Vec<Keypad> {
    code.chars().map(Keypad::from_char).collect()
}

#[allow(dead_code)]
fn parse_direction(code: &str) -> Vec<Direction> {
    code.chars().map(Direction::from_char).collect()
}

fn keypad_value(code: &[Keypad]) -> u32 {
    let filtered: String = code
        .iter()
        .filter(|key| **key != Keypad::A)
        .map(|key| key.to_string())
        .collect();
    filtered.parse::<u32>().unwrap()
}

fn get_user_cost_map() -> HashMap<(Direction, Direction), u32> {
    let mut out: HashMap<(Direction, Direction), u32> = HashMap::new();

    for a in Direction::types().iter().cloned() {
        for b in Direction::types().iter().cloned() {
            /* let (ax, ay) = a.get_point();
            let (bx, by) = b.get_point();

            let dx = (bx - ax).unsigned_abs() as u32;
            let dy = (by - ay).unsigned_abs() as u32;

            let cost = dx + dy;

            out.insert((a, b), cost); */
            out.insert((a, b), 0);
        }
    }

    out
}

fn get_robot_cost_map(
    costs: &HashMap<(Direction, Direction), u32>,
) -> HashMap<(Direction, Direction), u32> {
    let mut out: HashMap<(Direction, Direction), u32> = HashMap::new();

    for a in Direction::types().iter().cloned() {
        for b in Direction::types().iter().cloned() {
            let (ax, ay) = a.get_point();
            let (bx, by) = b.get_point();

            let dx = bx - ax;
            let dy = by - ay;

            let (x, a2) = match dx {
                i8::MIN..=-1 => {
                    let x = dx.unsigned_abs() as u32
                        + costs.get(&(Direction::A, Direction::Left)).unwrap();

                    (x, Direction::Left)
                }
                1..=i8::MAX => {
                    let x = dx.unsigned_abs() as u32
                        + costs.get(&(Direction::A, Direction::Right)).unwrap();

                    (x, Direction::Right)
                }
                _ => (0, Direction::A),
            };

            let y = match dy {
                i8::MIN..=-1 => {
                    dy.unsigned_abs() as u32
                        + costs.get(&(a2, Direction::Upp)).unwrap()
                        + costs.get(&(Direction::Upp, Direction::A)).unwrap()
                }
                1..=i8::MAX => {
                    dy.unsigned_abs() as u32
                        + costs.get(&(a2, Direction::Down)).unwrap()
                        + costs.get(&(Direction::Down, Direction::A)).unwrap()
                }
                _ => *costs.get(&(a2, Direction::A)).unwrap(),
            };

            let cost = x + y + 1;
            //println!("{:?} -> {:?} = {}", a, b, cost);

            out.insert((a, b), cost);
        }
    }

    out
}

fn get_keypad_cost_map(
    costs: &HashMap<(Direction, Direction), u32>,
) -> HashMap<(Keypad, Keypad), u32> {
    let mut out: HashMap<(Keypad, Keypad), u32> = HashMap::new();

    for a in Keypad::types().iter().cloned() {
        for b in Keypad::types().iter().cloned() {
            let (ax, ay) = a.get_point();
            let (bx, by) = b.get_point();

            let dx = bx - ax;
            let dy = by - ay;

            let (x, a2) = match dx {
                i8::MIN..=-1 => {
                    let x = dx.unsigned_abs() as u32
                        + costs.get(&(Direction::A, Direction::Left)).unwrap();

                    (x, Direction::Left)
                }
                1..=i8::MAX => {
                    let x = dx.unsigned_abs() as u32
                        + costs.get(&(Direction::A, Direction::Right)).unwrap();

                    (x, Direction::Right)
                }
                _ => (0, Direction::A),
            };

            let y = match dy {
                i8::MIN..=-1 => {
                    dy.unsigned_abs() as u32
                        + costs.get(&(a2, Direction::Upp)).unwrap()
                        + costs.get(&(Direction::Upp, Direction::A)).unwrap()
                }
                1..=i8::MAX => {
                    dy.unsigned_abs() as u32
                        + costs.get(&(a2, Direction::Down)).unwrap()
                        + costs.get(&(Direction::Down, Direction::A)).unwrap()
                }
                _ => *costs.get(&(a2, Direction::A)).unwrap(),
            };

            let cost = x + y + 1;
            //println!("{:?} -> {:?} = {}", a, b, cost);

            out.insert((a, b), cost);
        }
    }

    out
}

fn solve_keypad(costs: &HashMap<(Keypad, Keypad), u32>, code: &[Keypad]) -> u32 {
    code.iter()
        .enumerate()
        .map(|(i, to)| {
            let from = if i == 0 { Keypad::A } else { code[i - 1] };

            println!("> {}", costs.get(&(from, *to)).unwrap());

            costs.get(&(from, *to)).unwrap()
        })
        .sum()
}

fn solve(input: &str) -> u32 {
    let keypads: Vec<Vec<Keypad>> = input.lines().map(parse_keypad).collect();

    let user_cost = get_user_cost_map();
    let robot1_cost = get_robot_cost_map(&user_cost);
    //let robot2_cost = get_robot_cost_map(&robot1_cost);
    //let robot3_cost = get_robot_cost_map(&robot2_cost);
    let keypad_cost = get_keypad_cost_map(&robot1_cost);

    let solutions: Vec<u32> = keypads
        .iter()
        .map(|code| solve_keypad(&keypad_cost, code))
        .collect();
    let keypad_values: Vec<u32> = keypads.iter().map(|code| keypad_value(code)).collect();

    for sol in &solutions {
        println!("{}", sol);
        /*
        68
        60
        68
        64
        64
        */
    }

    let total: u32 = keypad_values
        .iter()
        .zip(solutions)
        .map(|(value, solution)| value * solution)
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    /* #[test]
        fn day21_part1_main() {
            let input = "029A
    980A
    179A
    456A
    379A";
            let output = solve(input);
            assert_eq!(output, 126384)
        } */

    #[test]
    fn day21_part1_test1() {
        let input = "029A";
        let output = solve(input);
        assert_eq!(output, 1972)
    }

    /* #[test]
    fn day21_part1_test2() {
        let input = "980A";
        let output = solve(input);
        assert_eq!(output, 58800)
    }

    #[test]
    fn day21_part1_test3() {
        let input = "179A";
        let output = solve(input);
        assert_eq!(output, 12172)
    }

    #[test]
    fn day21_part1_test4() {
        let input = "456A";
        let output = solve(input);
        assert_eq!(output, 29184)
    }

    #[test]
    fn day21_part1_test5() {
        let input = "379A";
        let output = solve(input);
        assert_eq!(output, 24256)
    } */
}
