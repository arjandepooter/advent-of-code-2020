const INPUT: &'static str = include_str!("./input.txt");

type Solution = u32;

fn solve_a(data: &str) -> Solution {
    todo!()
}

fn solve_b(data: &str) -> Solution {
    todo!()
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
