#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<&'a str>;
type Solution = usize;

fn parse_input(input: &str) -> Data {
    todo!()
}

fn solve_a(data: &Data) -> Solution {
    todo!()
}

fn solve_b(data: &Data) -> Solution {
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
    fn examples_a() {}

    #[test]
    fn examples_b() {}
}
