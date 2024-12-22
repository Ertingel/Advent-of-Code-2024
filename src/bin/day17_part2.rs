// cargo run  --bin day17_part2
// cargo test --bin day17_part2

use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day17.txt");
    let output = solve(input);
    println!("Day17 part2: {output}");
}

type Value = u64;

fn parse_input(data: &str) -> Option<(Value, Value, Value, Vec<u8>)> {
    let (data, _) = parsing::string("Register A: ")(data)?;
    let (data, reg_a) = parsing::number::<Value>()(data)?;

    let (data, _) = parsing::string("\nRegister B: ")(data)?;
    let (data, reg_b) = parsing::number::<Value>()(data)?;

    let (data, _) = parsing::string("\nRegister C: ")(data)?;
    let (data, reg_c) = parsing::number::<Value>()(data)?;

    let (data, _) = parsing::string("\n\nProgram: ")(data)?;
    let program: Vec<u8> = data.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    Some((reg_a, reg_b, reg_c, program))
}

/*
2 BXL 3(3)
4 CDV B(5)
6 ADV 3(3)
8 BXL A(4)
10 BXC Reserved(7)
12 OUT B(5)
14 JNZ 0(0)
*/
fn input_program(a: u64) -> Vec<u8> {
    let mut a = a;
    let mut out: Vec<u8> = Vec::new();

    while a != 0 {
        let mut b = a & 0b_111; // 0 BST A(4)
        b ^= 3; // 2 BXL 3(3)
        let c = a >> b; // 4 CDV B(5)
        a >>= 3; // 6 ADV 3(3)
        b ^= 4; // 8 BXL A(4)
        b ^= c; // 10 BXC Reserved(7)
        out.push(b as u8 & 0b_111); // 12 OUT B(5)
                                    // 14 JNZ 0(0)
    }

    out
}

fn output_to_number(output: &[u8]) -> Value {
    output
        .iter()
        .enumerate()
        .map(|(i, x)| (*x as Value) << (i * 5).min(54))
        .sum()
}

fn solve(input: &str) -> Value {
    let (_, _, _, original) = parse_input(input).unwrap();
    let target = output_to_number(&original);

    let mut testing = 0;
    let mut step_size = Value::MAX / 2;

    while step_size > 0 {
        let out = input_program(testing);
        let result = output_to_number(&out);

        if result == target {
            break;
        }

        if result < target {
            testing += step_size;
        } else {
            testing -= step_size;
        }
        step_size >>= 1;
    }

    let out = input_program(testing);
    //let result = output_to_number(&out);
    //assert_eq!(result, target);
    assert_eq!(out, original);

    testing

    /* let seach_space: Value = u64::MAX;
    let mut i = 1;

    while i <= seach_space {
        let out = input_program(i);

        if out == original {
            return i;
        }

        if i % 10_000_000 == 0 {
            println!(
                "Day17 part2: Seaching {} {:.1}%   len {}/{}",
                i,
                i as f32 / seach_space as f32 * 100.0,
                out.len(),
                original.len()
            )
        }
        //35801310000000
        //286410470000000
        let target_len = original.len();
        match out.len() {
            _ if out.len() < target_len - 1 => i *= 2,
            _ if out.len() == target_len - 1 => i = (i as f64 * 1.1) as Value,
            _ => i += 1,
        }
    } */

    /* let output: Value = 0;
    output */
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_part2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let output = solve(input);
        assert_eq!(output, 117440)
    }
} */
