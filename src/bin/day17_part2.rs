// cargo run  --bin day17_part2
// cargo test --bin day17_part2

use std::fmt;

use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day17.txt");
    let output = solve(input);
    println!("Day17 part2: {output}");
}

type Value = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Operand {
    /**Combo operands 0 represent literal values 0.*/
    Zero,
    /**Combo operands 1 represent literal values 1.*/
    One,
    /**Combo operands 2 represent literal values 2.*/
    Two,
    /**Combo operands 3 represent literal values 3.*/
    Three,

    /**Combo operand 4 represents the value of register A.*/
    RegA,
    /**Combo operand 5 represents the value of register B.*/
    RegB,
    /**Combo operand 6 represents the value of register C.*/
    RegC,

    /**Combo operand 7 is reserved and will not appear in valid programs.*/
    Reserved,
}

impl Operand {
    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Operand::Zero,
            1 => Operand::One,
            2 => Operand::Two,
            3 => Operand::Three,

            4 => Operand::RegA,
            5 => Operand::RegB,
            6 => Operand::RegC,

            7 => Operand::Reserved,
            _ => panic!(),
        }
    }

    fn value(&self, cpu: &Cpu) -> Value {
        match self {
            Operand::Zero => 0,
            Operand::One => 1,
            Operand::Two => 2,
            Operand::Three => 3,

            Operand::RegA => cpu.reg_a,
            Operand::RegB => cpu.reg_b,
            Operand::RegC => cpu.reg_c,

            Operand::Reserved => panic!(),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Zero => write!(f, "0"),
            Operand::One => write!(f, "1"),
            Operand::Two => write!(f, "2"),
            Operand::Three => write!(f, "3"),

            Operand::RegA => write!(f, "A"),
            Operand::RegB => write!(f, "B"),
            Operand::RegC => write!(f, "C"),

            Operand::Reserved => write!(f, "Reserved"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Instruction {
    /**
    The `adv` instruction (opcode `0`) performs **division**.
    The numerator is the value in the A register.
    The denominator is found by raising 2 to the power of the instruction's combo operand.
    (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    The result of the division operation is truncated to an integer and
    then written to the A register.
    */
    Adv,
    /**
    The `bxl` instruction (opcode `1`) calculates the **bitwise XOR** of register B and
    the instruction's literal operand, then stores the result in register B.
    */
    Bxl,
    /**
    The `bst` instruction (opcode `2`) calculates the value of its combo operand **modulo 8**
    (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    */
    Bst,
    /**
    The `jnz` instruction (opcode `3`) does nothing if the A register is 0.
    However, if the A register is not zero,
    it **jumps** by setting the instruction pointer to the value of its literal operand;
    if this instruction jumps,
    the instruction pointer is not increased by 2 after this instruction.
    */
    Jnz,
    /**
    The `bxc` instruction (opcode `4`) calculates the **bitwise XOR** of register B and register C,
    then stores the result in register B.
    (For legacy reasons, this instruction reads an operand but ignores it.)
    */
    Bxc,
    /**
    The `out` instruction (opcode `5`) calculates the value of its combo operand modulo 8,
    then **outputs** that value.
    (If a program outputs multiple values, they are separated by commas.)
    */
    Out,
    /**
    The `bdv` instruction (opcode `6`) works exactly like the adv(**division**) instruction except
    that the result is stored in the B register.
    (The numerator is still read from the A register.)
    */
    Bdv,
    /**
    The `cdv` instruction (opcode `7`) works exactly like the adv(**division**) instruction except
    that the result is stored in the C register.
    (The numerator is still read from the A register.)
    */
    Cdv,
}

impl Instruction {
    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!(),
        }
    }

    fn execute(&self, cpu: &mut Cpu, operand: Operand) {
        let literal = *cpu.get_byte(1).unwrap() as Value;

        match self {
            Instruction::Adv => {
                //cpu.reg_a =
                //    (cpu.reg_a as f32 / 2_f32.powf(operand.value(cpu) as f32)).floor() as Value;
                //cpu.reg_a /= 2_u32.pow(operand.value(cpu));
                cpu.reg_a >>= operand.value(cpu);
            }

            Instruction::Bxl => {
                //cpu.reg_b ^= operand.value(cpu);
                cpu.reg_b ^= literal;
            }

            Instruction::Bst => {
                //cpu.reg_b = operand.value(cpu) % 8;
                cpu.reg_b = operand.value(cpu) & 0b_111;
            }

            Instruction::Jnz => {
                if cpu.reg_a != 0 {
                    cpu.ins_pointer = literal as usize;
                    return;
                }
            }

            Instruction::Bxc => {
                cpu.reg_b ^= cpu.reg_c;
            }

            Instruction::Out => {
                // cpu.output.push((operand.value(cpu) % 8) as u8);
                cpu.output.push((operand.value(cpu) & 0b_111) as u8);
            }

            Instruction::Bdv => {
                //cpu.reg_b =
                //    (cpu.reg_a as f32 / 2_f32.powf(operand.value(cpu) as f32)).floor() as Value;
                //cpu.reg_b = cpu.reg_a / 2_u32.pow(operand.value(cpu));
                cpu.reg_b = cpu.reg_a >> operand.value(cpu);
            }

            Instruction::Cdv => {
                //cpu.reg_c =
                //    (cpu.reg_a as f32 / 2_f32.powf(operand.value(cpu) as f32)).floor() as Value;
                //cpu.reg_c = cpu.reg_a / 2_u32.pow(operand.value(cpu));
                cpu.reg_c = cpu.reg_a >> operand.value(cpu);
            }
        }

        cpu.ins_pointer += 2;
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Adv => write!(f, "ADV"),
            Instruction::Bxl => write!(f, "BXL"),
            Instruction::Bst => write!(f, "BST"),
            Instruction::Jnz => write!(f, "JNZ"),
            Instruction::Bxc => write!(f, "BXC"),
            Instruction::Out => write!(f, "OUT"),
            Instruction::Bdv => write!(f, "BDV"),
            Instruction::Cdv => write!(f, "CVD"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Cpu {
    /**
    A number called the instruction pointer identifies the position in
    the program from which the next opcode will be read.
    If the computer tries to read an opcode past the end of the program, it instead halts.
     */
    ins_pointer: usize,

    /**Register A. */
    reg_a: Value,
    /**Register B. */
    reg_b: Value,
    /**Register C. */
    reg_c: Value,

    /**Program buffer. */
    program: Vec<u8>,

    /**Output buffer. */
    output: Vec<u8>,
}

impl Cpu {
    fn from_string(data: &str) -> Option<Self> {
        let (data, _) = parsing::string("Register A: ")(data)?;
        let (data, reg_a) = parsing::number::<Value>()(data)?;

        let (data, _) = parsing::string("\nRegister B: ")(data)?;
        let (data, reg_b) = parsing::number::<Value>()(data)?;

        let (data, _) = parsing::string("\nRegister C: ")(data)?;
        let (data, reg_c) = parsing::number::<Value>()(data)?;

        let (data, _) = parsing::string("\n\nProgram: ")(data)?;
        let program: Vec<u8> = data.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
        let output: Vec<u8> = Vec::new();

        Some(Cpu {
            ins_pointer: 0,
            reg_a,
            reg_b,
            reg_c,
            program,
            output,
        })
    }

    fn run(&mut self) {
        while self.execute().is_some() {}
    }

    fn execute(&mut self) -> Option<()> {
        let instruction = Instruction::from_byte(*self.get_byte(0)?);
        let operand = Operand::from_byte(*self.get_byte(1)?);

        //println!("{} {} {}", self.ins_pointer, instruction, operand,);

        instruction.execute(self, operand);

        Some(())
    }

    fn get_byte(&self, offset: usize) -> Option<&u8> {
        self.program.get(self.ins_pointer + offset)
    }

    /* fn output(&self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    } */
}

fn solve(input: &str) -> Value {
    let mut cpu = Cpu::from_string(input).unwrap();

    cpu.run();

    let output: Value = 0;

    output
}

#[cfg(test)]
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
}
