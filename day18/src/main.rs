#![feature(or_patterns)]
#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<&'a str>;
type Solution = u64;

fn parse_input(input: &str) -> Data {
    input.lines().collect()
}

trait Operator: std::fmt::Debug {
    fn evaluate(&self, lhs: u64, rhs: u64) -> u64;

    fn get_precedence(&self) -> u64 {
        0
    }
}
#[derive(Debug)]
struct Add;
#[derive(Debug)]
struct Mul;

impl Operator for Add {
    fn evaluate(&self, lhs: u64, rhs: u64) -> u64 {
        lhs + rhs
    }
}

impl Operator for Mul {
    fn evaluate(&self, lhs: u64, rhs: u64) -> u64 {
        lhs * rhs
    }
}

#[derive(Debug)]
struct PrioAdd;

impl Operator for PrioAdd {
    fn evaluate(&self, lhs: u64, rhs: u64) -> u64 {
        lhs + rhs
    }

    fn get_precedence(&self) -> u64 {
        10
    }
}

#[derive(Debug)]
enum OpEntry<'a> {
    Operator(&'a Box<dyn Operator>),
    Bracket,
}

fn pop_operator(output_stack: &mut Vec<u64>, op_stack: &mut Vec<OpEntry>) {
    let rhs = output_stack.pop().unwrap();
    let lhs = output_stack.pop().unwrap();
    let operator = op_stack.pop().unwrap();

    if let OpEntry::Operator(operator) = operator {
        output_stack.push(operator.evaluate(lhs, rhs));
    }
}

fn evaluate(s: &str, operators: &HashMap<char, Box<dyn Operator>>) -> u64 {
    let mut output_stack = vec![];
    let mut op_stack: Vec<OpEntry> = vec![];

    for c in s.chars() {
        match c {
            ' ' => {}
            '0'..='9' => {
                output_stack.push(c.to_digit(10).unwrap() as u64);
            }
            c if operators.contains_key(&c) => {
                let cur_op = operators.get(&c).unwrap();

                while let Some(OpEntry::Operator(prev_op)) = op_stack.last() {
                    if prev_op.get_precedence() >= cur_op.get_precedence() {
                        pop_operator(&mut output_stack, &mut op_stack);
                    } else {
                        break;
                    }
                }
                op_stack.push(OpEntry::Operator(cur_op));
            }
            '(' => op_stack.push(OpEntry::Bracket),
            ')' => loop {
                match op_stack.last() {
                    Some(OpEntry::Bracket) => {
                        op_stack.pop();
                        break;
                    }
                    Some(_) => pop_operator(&mut output_stack, &mut op_stack),
                    _ => break,
                }
            },
            c => panic!("INVALID CHAR: {}", c),
        }
    }

    while op_stack.len() > 0 {
        pop_operator(&mut output_stack, &mut op_stack);
    }

    output_stack[0]
}

fn solve_a(data: &Data) -> Solution {
    let mut operators: HashMap<char, Box<dyn Operator>> = HashMap::new();
    operators.insert('+', Box::new(Add));
    operators.insert('*', Box::new(Mul));

    data.iter().map(|s| evaluate(*s, &operators)).sum()
}

fn solve_b(data: &Data) -> Solution {
    let mut operators: HashMap<char, Box<dyn Operator>> = HashMap::new();
    operators.insert('+', Box::new(PrioAdd));
    operators.insert('*', Box::new(Mul));

    data.iter().map(|s| evaluate(*s, &operators)).sum()
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
    fn solve_a() {
        let mut operators: HashMap<char, Box<dyn Operator>> = HashMap::new();
        operators.insert('+', Box::new(Add));
        operators.insert('*', Box::new(Mul));

        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", &operators), 437);
    }

    #[test]
    fn solve_b() {
        let mut operators: HashMap<char, Box<dyn Operator>> = HashMap::new();
        operators.insert('+', Box::new(PrioAdd));
        operators.insert('*', Box::new(Mul));

        assert_eq!(
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", &operators),
            669060
        );
    }
}
