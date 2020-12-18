#![feature(or_patterns)]
#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<&'a str>;
type Solution = u64;

fn parse_input(input: &str) -> Data {
    input.lines().collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StackEntry {
    Mul,
    Add,
    OpenBracket,
    Value(u64),
}
use StackEntry::*;

fn push_value(stack: &mut Vec<StackEntry>, value: u64) {
    match stack.last() {
        Some(Mul | Add) => {
            let op = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            stack.push(Value(match (lhs, op) {
                (Value(lhs), Add) => value + lhs,
                (Value(lhs), Mul) => value * lhs,
                _ => 0,
            }))
        }
        _ => stack.push(Value(value)),
    }
}

fn evaluate_expression(expr: &str) -> u64 {
    let mut stack = vec![];

    for c in expr.chars() {
        match c {
            '(' => stack.push(OpenBracket),
            ')' => {
                let val = stack.pop().unwrap();
                stack.pop();
                if let Value(val) = val {
                    push_value(&mut stack, val);
                }
            }
            '*' => stack.push(Mul),
            '+' => stack.push(Add),
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap();
                push_value(&mut stack, digit as u64);
            }
            _ => (),
        }
    }

    match stack.get(0) {
        Some(Value(n)) => *n,
        _ => 0,
    }
}

fn solve_a(data: &Data) -> Solution {
    data.iter().map(|s| evaluate_expression(*s)).sum()
}

fn solve_b(_data: &Data) -> Solution {
    todo!()
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate() {
        let expr = "3 * 7 * (4 + 3)";

        assert_eq!(evaluate_expression(expr), 147);
    }
}
