// cargo run  --bin day21_part1
// cargo test --bin day21_part1

use core::fmt;

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

fn move_axis(
    out: &mut Vec<Direction>,
    from: i8,
    to: i8,
    decrement: Direction,
    increment: Direction,
) {
    let delta = to - from;
    match delta {
        1..=i8::MAX => {
            for _ in 0..delta {
                out.push(increment);
            }
        }

        i8::MIN..=-1 => {
            for _ in 0..-delta {
                out.push(decrement);
            }
        }

        _ => {}
    }
}

fn solve_keypad(code: &[Keypad]) -> Vec<Direction> {
    let mut at: Point = Keypad::A.get_point();
    let mut out: Vec<Direction> = Vec::new();

    for key in code {
        let target = key.get_point();

        if at.0 == 3 && target.0 == 0 {
            move_axis(&mut out, at.1, target.1, Direction::Upp, Direction::Down);
            move_axis(&mut out, at.0, target.0, Direction::Left, Direction::Right);
        } else {
            move_axis(&mut out, at.0, target.0, Direction::Left, Direction::Right);
            move_axis(&mut out, at.1, target.1, Direction::Upp, Direction::Down);
        }

        out.push(Direction::A);
        at = target;
    }

    out
}

fn solve_direction(directions: &[Direction]) -> Vec<Direction> {
    let mut at: Point = Direction::A.get_point();
    let mut out: Vec<Direction> = Vec::new();

    for key in directions {
        let target = key.get_point();

        if at.1 == 0 && target.0 == 0 {
            move_axis(&mut out, at.0, target.0, Direction::Left, Direction::Right);
            move_axis(&mut out, at.1, target.1, Direction::Upp, Direction::Down);
        } else {
            move_axis(&mut out, at.1, target.1, Direction::Upp, Direction::Down);
            move_axis(&mut out, at.0, target.0, Direction::Left, Direction::Right);
        }

        out.push(Direction::A);
        at = target;
    }

    out
}

fn solve(input: &str) -> u32 {
    let keypads: Vec<Vec<Keypad>> = input.lines().map(parse_keypad).collect();

    let solutions: Vec<Vec<Direction>> = keypads
        .iter()
        .map(|code| solve_direction(&solve_direction(&solve_keypad(code))))
        .collect();

    let keypad_values: Vec<u32> = keypads.iter().map(|code| keypad_value(code)).collect();

    for sol in &solutions {
        println!(
            "{:?} {}",
            sol.len(),
            sol.iter().map(|c| c.to_string()).collect::<String>()
        );
    }

    let total: u32 = keypad_values
        .iter()
        .zip(solutions)
        .map(|(value, solution)| value * solution.len() as u32)
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_part1_main() {
        let input = "029A
980A
179A
456A
379A";
        let output = solve(input);
        assert_eq!(output, 126384)
    }

    #[test]
    fn day21_part1_test1() {
        let result = solve_keypad(&parse_keypad("029A"));

        if result == parse_direction("<A^A>^^AvvvA") {
            return;
        }

        if result == parse_direction("<A^A^>^AvvvA") {
            return;
        }

        if result == parse_direction("<A^A^^>AvvvA") {
            return;
        }

        panic!()
    }

    #[test]
    fn day21_part1_test2() {
        let result = solve_direction(&solve_keypad(&parse_keypad("029A")));
        /* println!(
            "{}",
            result.iter().map(|x| x.to_string()).collect::<String>()
        ); */
        assert_eq!(result.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
    }

    #[test]
    fn day21_part1_test3() {
        let result = solve_direction(&solve_direction(&solve_keypad(&parse_keypad("029A"))));
        /* println!(
            "{}",
            result.iter().map(|x| x.to_string()).collect::<String>()
        ); */
        assert_eq!(
            result.len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn day21_part1_test3_1() {
        let input = "029A";
        let output = solve(input);
        assert_eq!(output, 1972)
    }

    #[test]
    fn day21_part1_test3_2() {
        let input = "980A";
        let output = solve(input);
        assert_eq!(output, 58800)
    }

    #[test]
    fn day21_part1_test3_3() {
        let input = "179A";
        let output = solve(input);
        assert_eq!(output, 12172)
    }

    #[test]
    fn day21_part1_test3_4() {
        let input = "456A";
        let output = solve(input);
        assert_eq!(output, 29184)
    }

    #[test]
    fn day21_part1_test3_5() {
        let input = "379A";
        let output = solve(input);
        assert_eq!(output, 24256)
    }
}

//<<vAA>A^>AA<Av>A^AvA^A<<vA^>>AAvA^Av<A^>AA<A>Av<A<A^>>AAA<Av>A^A
//<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

//<<vAA>A^>AA<Av>A^AAvA^Av<A^>A<A>Av<A^>A<A>Av<A<A^>>AA<Av>A^A
//<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
