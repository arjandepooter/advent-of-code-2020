#![feature(iterator_fold_self)]
use std::collections::HashSet;

const INPUT: &'static str = include_str!("./input.txt");

type Solution = usize;

fn parse_block(block: &str) -> Vec<HashSet<char>> {
    block.lines().map(|block| block.chars().collect()).collect()
}

fn parse_input(data: &str) -> Vec<Vec<HashSet<char>>> {
    data.split("\n\n").map(parse_block).collect()
}

fn solve_a(data: &str) -> Solution {
    let answer_groups = parse_input(data);

    answer_groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold(HashSet::new(), |result, answers| &answers | &result)
                .len()
        })
        .sum()
}

fn solve_b(data: &str) -> Solution {
    let answer_groups = parse_input(data);

    answer_groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold_first(|result, answers| &answers & &result)
                .unwrap()
                .len()
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", solve_a(INPUT));
    println!("Part 2: {}", solve_b(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_a() {
        assert_eq!(solve_a(""), 0);
    }

    #[test]
    fn examples_b() {
        assert_eq!(solve_b(""), 0);
    }
}
