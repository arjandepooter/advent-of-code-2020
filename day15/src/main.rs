#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = "12,20,0,6,1,17,7";

type Data = Vec<u64>;
type Solution = u64;

fn parse_input(input: &str) -> Data {
    input.split(",").filter_map(|n| n.parse().ok()).collect()
}

struct Game {
    mem: HashMap<u64, usize>,
    idx: usize,
    initial: Vec<u64>,
    last: u64,
}

impl Game {
    fn new(initial: &Vec<u64>) -> Self {
        Self {
            idx: 0,
            last: 0,
            mem: HashMap::new(),
            initial: initial.clone(),
        }
    }
}

impl Iterator for Game {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = if self.idx < self.initial.len() {
            self.initial[self.idx]
        } else {
            self.mem
                .get(&self.last)
                .map(|n| (self.idx - 1 - *n) as u64)
                .unwrap_or(0)
        };

        if self.idx > 0 {
            self.mem.insert(self.last, self.idx - 1);
        }
        self.idx += 1;
        self.last = current;

        Some(current)
    }
}

fn solve_a(data: &Data) -> Solution {
    Game::new(&data).nth(2020 - 1).unwrap()
}

fn solve_b(data: &Data) -> Solution {
    Game::new(&data).nth(30000000 - 1).unwrap()
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "0,3,6";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_a(&data), 436);
    }
}
