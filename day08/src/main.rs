#![feature(test)]
#![feature(str_split_once)]
#[allow(unused_imports)]
use shared::prelude::*;

extern crate test;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<Instruction>;
type Solution = i32;

#[derive(Debug, Clone, PartialEq, Copy)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone, Copy)]
struct Instruction(OpCode, i32);

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let (op_code, value) = line.split_once(' ').unwrap();
            let op_code = match op_code {
                "jmp" => OpCode::Jmp,
                "acc" => OpCode::Acc,
                _ => OpCode::Nop,
            };
            let value = value.parse().unwrap();

            Instruction(op_code, value)
        })
        .collect()
}

fn run_program(program: &Data) -> (i32, usize, Vec<usize>) {
    let mut pointer: usize = 0;
    let mut acc: i32 = 0;
    let mut seen = vec![false; program.len()];

    while pointer < program.len() && !seen[pointer] {
        let instruction = program.get(pointer).unwrap();
        seen[pointer] = true;

        match instruction {
            Instruction(OpCode::Jmp, value) => {
                pointer = (pointer as i32 + value).max(0) as usize;
            }
            Instruction(OpCode::Acc, value) => {
                pointer += 1;
                acc += value;
            }
            Instruction(OpCode::Nop, _) => {
                pointer += 1;
            }
        }
    }

    (
        acc,
        pointer,
        seen.into_iter()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(idx, _)| idx)
            .collect(),
    )
}

fn solve_a(data: &Data) -> Solution {
    let (acc, _, _) = run_program(&data);

    acc
}

type DiGraph = HashMap<usize, Vec<usize>>;

fn build_endpoint_graph(data: &Data) -> DiGraph {
    let mut graph: DiGraph = HashMap::new();

    for (idx, Instruction(op_code, value)) in data.into_iter().enumerate() {
        let target: usize = if *op_code == OpCode::Jmp {
            (idx as i32 + value).max(0) as usize
        } else {
            idx + 1
        };

        let target = target.min(data.len());
        graph.entry(target).or_insert_with(|| Vec::new()).push(idx);
    }

    graph
}

fn reachable_nodes(graph: &DiGraph, start: usize, seen: &mut Vec<bool>) -> Vec<usize> {
    let mut descendants: Vec<usize> = Vec::new();
    descendants.push(start);

    seen[start] = true;

    match graph.get(&start) {
        Some(children) => {
            children.into_iter().for_each(|idx| {
                if !seen[*idx] {
                    let b = reachable_nodes(graph, *idx, seen);
                    descendants.extend(b);
                }
            });
        }
        None => (),
    }

    descendants
}

fn find_swap(data: &Data, end_nodes: &Vec<usize>) -> usize {
    let (_, _, start_nodes) = run_program(data);
    for idx in start_nodes {
        let Instruction(op_code, value) = data.get(idx).unwrap();

        match *op_code {
            OpCode::Nop | OpCode::Jmp => {
                let target = if *op_code == OpCode::Nop {
                    (idx as i32 + value).min(data.len() as i32) as usize
                } else {
                    idx + 1
                };
                if end_nodes.contains(&target) {
                    return idx;
                }
            }
            _ => (),
        }
    }

    0
}

// Find all instructions connected to the endpoint, loop through the instructions
// seen in the first run and check for every instruction if the target of the swap
// ends up in the set of endpoint connected nodes.
fn solve_b(data: &Data) -> Solution {
    let graph = build_endpoint_graph(data);
    let end_nodes = reachable_nodes(&graph, data.len(), &mut vec![false; data.len() + 1]);
    let swap_idx = find_swap(data, &end_nodes);

    let mut cloned_program = data.clone();
    cloned_program[swap_idx] = match data[swap_idx] {
        Instruction(OpCode::Nop, value) => Instruction(OpCode::Jmp, value),
        Instruction(OpCode::Jmp, value) => Instruction(OpCode::Nop, value),
        nop => nop,
    };

    let (acc, _, _) = run_program(&cloned_program);
    acc
}

// Check all seen `jmp` and `nop` instructions, swap them, run program again
// and check if it finished without a loop
fn solve_b_brute_force(data: &Data) -> Solution {
    let (_, _, seen) = run_program(data);
    for idx in seen {
        match data.get(idx) {
            Some(Instruction(op_code, value))
                if *op_code == OpCode::Nop || *op_code == OpCode::Jmp =>
            {
                let mut cloned = data.clone();
                let op_code = if *op_code == OpCode::Jmp {
                    OpCode::Nop
                } else {
                    OpCode::Jmp
                };
                cloned[idx] = Instruction(op_code, *value);
                let (acc, pointer, _) = &run_program(&cloned);
                if *pointer >= cloned.len() {
                    return *acc;
                }
            }
            _ => {}
        }
    }
    0
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
    println!("Part B (bruteforce): {}", solve_b_brute_force(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_a(&data), 5);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_b(&data), 8);
    }

    #[test]
    fn examples_b_bruteforce() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_b_brute_force(&data), 8);
    }

    #[bench]
    fn bench_a(b: &mut Bencher) {
        b.iter(|| {
            let data = parse_input(INPUT);
            solve_a(&data)
        })
    }

    #[bench]
    fn bench_b(b: &mut Bencher) {
        let data = parse_input(INPUT);
        b.iter(|| build_endpoint_graph(&data))
    }

    #[bench]
    fn bench_b_bruteforce(b: &mut Bencher) {
        b.iter(|| {
            let data = parse_input(INPUT);
            solve_b_brute_force(&data)
        })
    }
}
