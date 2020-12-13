#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data = (i128, Vec<Option<i128>>);
type Solution = i128;

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    let target = lines.next().and_then(|line| line.parse().ok()).unwrap_or(0);
    let busses = lines
        .next()
        .unwrap()
        .split(",")
        .map(|bus| match bus {
            "x" => None,
            n => n.parse().ok(),
        })
        .collect();

    (target, busses)
}

fn solve_a((target, busses): &Data) -> Solution {
    busses
        .iter()
        .filter_map(|bus| match bus {
            Some(id) => Some((id, id - target % id)),
            None => None,
        })
        .min_by_key(|(_, offset)| *offset)
        .map_or(0, |(id, offset)| id * offset)
}

fn inverse(n: i128, x: i128) -> i128 {
    (0..x).find(|i| i * n % x == 1).unwrap_or(0)
}

fn solve_b((_, busses): &Data) -> Solution {
    let busses: Vec<_> = busses
        .iter()
        .enumerate()
        .filter_map(|(idx, bus)| bus.map(|id| (idx, id)))
        .collect();

    // Chinese Remainder Theorem
    let n: i128 = busses.iter().map(|(_, n)| n).product();

    let total: i128 = busses
        .iter()
        .map(|(idx, bus)| {
            let b = (bus - *idx as i128) % bus;
            let ni = n / bus;
            let x = inverse(ni, *bus);

            b * ni * x
        })
        .sum();

    total % n
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_a(&data), 295);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_b(&data), 1068781);
    }
}
