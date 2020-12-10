#![feature(test)]
#[allow(unused_imports)]
use shared::prelude::*;

use std::iter::once;

extern crate test;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<usize>;
type Solution = usize;

fn parse_input(input: &str) -> Data {
    input.lines().filter_map(|n| n.parse().ok()).collect()
}

fn number_of_combinations(lst: &Data, start: usize, mem: &mut HashMap<usize, usize>) -> usize {
    let max = lst.iter().max().unwrap();

    if mem.contains_key(&start) {
        return *mem.get(&start).unwrap();
    }

    let result = match start {
        start if start == *max => 1,
        start if start > *max => 0,
        start => (1..=3)
            .into_iter()
            .filter(|offset| lst.contains(&(start + offset)))
            .map(|offset| number_of_combinations(lst, start + offset, mem))
            .sum(),
    };

    mem.insert(start, result);

    result
}

fn solve_a(data: &Data) -> Solution {
    let mut seq = data.clone();
    seq.sort();

    let endpoint = seq.iter().max().unwrap() + 3;
    let iter_start = once(&0).chain(seq.iter());
    let iter_end = seq.iter().chain(once(&endpoint));

    let (ones, threes) = iter_start
        .zip(iter_end)
        .fold((0, 0), |(ones, threes), (a, b)| match b - a {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });

    ones * threes
}

fn solve_b(data: &Data) -> Solution {
    let mut mem = HashMap::with_capacity(data.len());

    number_of_combinations(data, 0, &mut mem)
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const EXAMPLE: &'static str = "16
10
15
5
1
11
7
19
6
12
4";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_a(&data), 35);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_b(&data), 8);
    }

    #[bench]
    fn bench_b(b: &mut Bencher) {
        let data = parse_input(INPUT);

        b.iter(|| solve_b(&data))
    }
}
