#[allow(unused_imports)]
use shared::prelude::*;
use std::str::FromStr;
use strum_macros::EnumString;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Program;
type Solution = i32;

#[derive(EnumString, Debug, Clone)]
#[strum(serialize_all = "lowercase")]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
struct Instruction(OpCode, i32);

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op_code, value) = s.split_at(3);
        let op_code: OpCode = op_code.parse().map_err(|_| ())?;
        let value: i32 = value.trim().parse().map_err(|_| ())?;

        Ok(Self(op_code, value))
    }
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> =
            s.lines().filter_map(|line| line.parse().ok()).collect();

        Ok(Self { instructions })
    }
}

impl Program {
    fn run(&self) -> Result<i32, i32> {
        let mut acc = 0;
        let mut pointer: usize = 0;
        let mut seen: HashSet<usize> = HashSet::new();

        while !seen.contains(&pointer) && pointer < self.instructions.len() {
            let instruction = self.instructions.get(pointer).unwrap();
            seen.insert(pointer);

            match instruction {
                Instruction(OpCode::Nop, _) => {
                    pointer += 1;
                }
                Instruction(OpCode::Acc, value) => {
                    acc += value;
                    pointer += 1;
                }
                Instruction(OpCode::Jmp, value) => {
                    let offset = *value.clamp(
                        &-(pointer as i32),
                        &((self.instructions.len() - pointer) as i32),
                    );
                    pointer = ((pointer as i32) + offset) as usize;
                }
            }
        }

        if seen.contains(&pointer) {
            Err(acc)
        } else {
            Ok(acc)
        }
    }
}

fn solve_a(data: &Data) -> Solution {
    data.run().unwrap_err()
}

fn solve_b(data: &Data) -> Solution {
    data.instructions
        .iter()
        .enumerate()
        .find_map(|(idx, instruction)| match instruction {
            Instruction(OpCode::Acc, _) => None,
            Instruction(op_code, value) => {
                let mut program = data.clone();
                program.instructions[idx] = match op_code {
                    OpCode::Jmp => Instruction(OpCode::Nop, *value),
                    _ => Instruction(OpCode::Jmp, *value),
                };
                program.run().ok()
            }
        })
        .unwrap()
}

fn main() {
    let data = INPUT.parse().unwrap();

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn examples_a() {
        let data = EXAMPLE.parse::<Program>().unwrap();

        assert_eq!(solve_a(&data), 5);
    }

    #[test]
    fn examples_b() {
        let data = EXAMPLE.parse::<Program>().unwrap();

        assert_eq!(solve_b(&data), 8);
    }
}
