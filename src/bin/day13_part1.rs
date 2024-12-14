// cargo run  --bin day13_part1
// cargo test --bin day13_part1

use std::collections::HashMap;

use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day13.txt");
    let output = solve(input);
    println!("Day13 part1: {output}");
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Button {
    movement: HashMap<char, i32>,
    cost: i32,
}

impl Button {
    fn parse_axes(data: &str) -> Option<(&str, (char, i32))> {
        let (data, axis) = parsing::alpha()(data)?;
        let (data, sign) = parsing::any(parsing::char('+'), parsing::char('-'))(data)?;
        let (data, mut amount) = parsing::number::<i32>()(data)?;
        let (data, _) = parsing::optional(parsing::string(", "))(data)?;

        let axis = axis.chars().next()?;
        if sign.starts_with('-') {
            amount = -amount;
        }

        Some((data, (axis, amount)))
    }

    fn from_string(data: &str) -> Option<(&str, (char, Self))> {
        //Button A: X+94, Y+34

        let (data, _) = parsing::string("Button ")(data)?;
        let (data, label) = parsing::uppercase()(data)?;
        let (data, _) = parsing::string(": ")(data)?;
        let (data, movement) = parsing::repeating(1, 10, Button::parse_axes)(data)?;
        let (data, _) = parsing::char('\n')(data)?;

        let label = label.chars().next()?;
        let movement: HashMap<char, i32> = movement.iter().cloned().collect();

        Some((data, (label, Self { movement, cost: 1 })))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ClawMachine {
    buttons: HashMap<char, Button>,
    prize: HashMap<char, i32>,
}

impl ClawMachine {
    fn parse_axes(data: &str) -> Option<(&str, (char, i32))> {
        let (data, axis) = parsing::alpha()(data)?;
        let (data, _) = parsing::char('=')(data)?;
        let (data, amount) = parsing::number::<i32>()(data)?;
        let (data, _) = parsing::optional(parsing::string(", "))(data)?;

        let axis = axis.chars().next()?;
        Some((data, (axis, amount)))
    }

    fn from_string(data: &str) -> Option<(&str, Self)> {
        //Button A: X+94, Y+34
        //Button B: X+22, Y+67
        //Prize: X=8400, Y=5400

        let (data, buttons) = parsing::repeating(1, 10, Button::from_string)(data)?;
        let (data, _) = parsing::string("Prize: ")(data)?;
        let (data, prize) = parsing::repeating(1, 10, ClawMachine::parse_axes)(data)?;

        let mut buttons: HashMap<char, Button> = buttons.iter().cloned().collect();
        let prize: HashMap<char, i32> = prize.iter().cloned().collect();

        buttons.get_mut(&'A').unwrap().cost = 3;
        buttons.get_mut(&'B').unwrap().cost = 1;

        Some((data, ClawMachine { buttons, prize }))
    }

    fn get_solution(&self) -> Option<(i32, i32)> {
        /*
        # Solving system of linear equations

        ax * a + bx * b = x
        ay * a + by * b = y


        # Solving for "b"

        ( ax * a + bx * b = x ) * ay
         => ax * ay * a + bx * ay * b = ay * x

        ( ay * a + by * b = y ) * ax
         => ax * ay * a + ax * by * b = ax * y

        ( ax * ay * a + bx * ay * b = ay * x ) - ( ax * ay * a + ax * by * b = ax * y )
         => ( bx * ay * b = ay * x ) - ( ax * by * b = ax * y )
         => bx * ay * b - ax * by * b = ay * x - ax * y
         => (bx * ay - ax * by) * b = ay * x - ax * y

        ( (bx * ay - ax * by) * b = ay * x - ax * y ) / (bx * ay - ax * by)
         => b = (ay * x - ax * y) / (bx * ay - ax * by)


        # Solving for "a"

        ( ax * a + bx * b = x ) - bx * b
         => ax * a = x - bx * b

        ( ax * a = x - bx * b ) / ax
         => a = (x - bx * b) / ax

        */

        let a = self.buttons.get(&'A').unwrap();
        let b = self.buttons.get(&'B').unwrap();

        let ax = a.movement.get(&'X').unwrap();
        let ay = a.movement.get(&'Y').unwrap();

        let bx = b.movement.get(&'X').unwrap();
        let by = b.movement.get(&'Y').unwrap();

        let x = self.prize.get(&'X').unwrap();
        let y = self.prize.get(&'Y').unwrap();

        //  b = (ay * x - ax * y) / (bx * ay - ax * by)
        let b_numerator = ay * x - ax * y;
        let b_denominator = bx * ay - ax * by;

        if b_numerator % b_denominator != 0 {
            return None;
        }
        let b = b_numerator / b_denominator;

        // a = (x - bx * b) / ax
        let a_numerator = x - bx * b;
        let a_denominator = ax;

        if a_numerator % a_denominator != 0 {
            return None;
        }
        let a = a_numerator / a_denominator;

        Some((a, b))
    }

    fn get_solution_cost(&self) -> Option<i32> {
        let solution = self.get_solution()?;

        let a = self.buttons.get(&'A').unwrap().cost;
        let b = self.buttons.get(&'B').unwrap().cost;

        Some(a * solution.0 + b * solution.1)
    }
}

fn solve(input: &str) -> i32 {
    let arcade: Vec<ClawMachine> = input
        .split("\n\n")
        .map(|data| ClawMachine::from_string(data).unwrap().1)
        .collect();

    let total: i32 = arcade
        .iter()
        .filter_map(|machine| machine.get_solution_cost())
        .sum();

    //println!("{:?}", arcade);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let output = solve(input);
        assert_eq!(output, 480)
    }
}
