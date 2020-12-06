#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data = Vec<Vec<u32>>;

fn parse_input(data: &str) -> Data {
    data.blocks()
        .map(|block| block.lines().map(line_bitmask).collect())
        .collect()
}

fn line_bitmask(line: &str) -> u32 {
    line.bytes()
        .fold(0u32, |mask, c| mask | (1 << c - ('a' as u8)))
}

fn compare_answers(groups: &Data, f: fn(u32, u32) -> u32, initial: u32) -> u32 {
    groups
        .iter()
        .map(|block| block.into_iter().map(|n| *n).fold(initial, f))
        .map(|n| n.count_ones())
        .sum()
}

fn solve_a(data: &Data) -> u32 {
    compare_answers(data, |a, b| a | b, 0)
}

fn solve_b(data: &Data) -> u32 {
    compare_answers(data, |a, b| a & b, !0)
}

fn main() {
    let groups = parse_input(INPUT);

    println!("Part A: {}", solve_a(&groups));
    println!("Part B: {}", solve_b(&groups));
}
