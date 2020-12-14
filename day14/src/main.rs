#![feature(str_split_once)]
#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<(Vec<char>, Vec<(u64, u64)>)>;
type Solution = u64;

fn parse_mask(line: &str) -> Vec<char> {
    line.trim_start_matches("mask = ").chars().rev().collect()
}

fn parse_assignment(line: &str) -> (u64, u64) {
    let (target, value) = line.split_once(" = ").unwrap();
    let target = target
        .trim_start_matches("mem[")
        .trim_end_matches("]")
        .parse()
        .unwrap();
    let value = value.parse().unwrap();

    (target, value)
}

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    let mut current_mask = parse_mask(lines.next().unwrap());
    let mut assignments = vec![];
    let mut result = vec![];

    for line in lines {
        if line.starts_with("mask = ") {
            result.push((current_mask, assignments));
            assignments = vec![];
            current_mask = parse_mask(line);
        } else if line.starts_with("mem[") {
            let assignment = parse_assignment(line);
            assignments.push(assignment);
        }
    }
    result.push((current_mask, assignments));

    result
}

fn apply_mask(mask: &Vec<char>, value: u64) -> u64 {
    mask.iter().enumerate().fold(0, |acc, (idx, c)| match *c {
        '1' => acc | 1 << idx,
        'X' => acc | ((value >> idx) & 1) << idx,
        _ => acc,
    })
}

fn expand_mask(mask: &Vec<char>, value: u64) -> Vec<u64> {
    let xs = mask.iter().filter(|c| **c == 'X').count() as u32;

    (0..(2u64.pow(xs)))
        .map(|n| {
            mask.iter()
                .enumerate()
                .fold((0, n), |(acc, n), (idx, c)| match c {
                    '1' => (acc | 1 << idx, n),
                    'X' => (acc | (n & 1) << idx, n >> 1),
                    _ => (acc | ((value >> idx) & 1) << idx, n),
                })
                .0
        })
        .collect()
}

fn solve_a(data: &Data) -> Solution {
    let mut mem = HashMap::with_capacity(data.iter().map(|(_, ass)| ass.len()).sum());

    for (mask, assignments) in data {
        for (target, value) in assignments {
            mem.insert(*target, apply_mask(mask, *value));
        }
    }

    mem.values().sum()
}

fn solve_b(data: &Data) -> Solution {
    let mut mem = HashMap::new();
    let mut result = 0;

    for (mask, assignments) in data {
        for (target, value) in assignments {
            for address in expand_mask(mask, *target) {
                let old = mem.insert(address, *value);
                result += *value;
                result -= old.unwrap_or(0);
            }
        }
    }

    result
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
    fn examples_a() {
        let data = parse_input(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
        );

        assert_eq!(solve_a(&data), 165);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(
            "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        );
        assert_eq!(solve_b(&data), 208);
    }
}
