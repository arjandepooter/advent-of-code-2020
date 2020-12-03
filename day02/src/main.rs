#![feature(str_split_once)]
use std::str::FromStr;

const INPUT: &'static str = include_str!("./input.txt");

type Solution = usize;

struct Password {
    password: String,
    start: usize,
    stop: usize,
    char: char,
}

struct ParseError;

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (policy, password) = s.split_once(": ").ok_or(ParseError)?;
        let (range, s_char) = policy.split_once(' ').ok_or(ParseError)?;
        let (s_start, s_stop) = range.split_once('-').ok_or(ParseError)?;
        let start = s_start.parse::<usize>().map_err(|_| ParseError)?;
        let stop = s_stop.parse::<usize>().map_err(|_| ParseError)?;
        let char = s_char.chars().next().ok_or(ParseError)?;

        Ok(Password {
            password: password.to_string(),
            char,
            start,
            stop,
        })
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.char).count();

        count >= self.start && count <= self.stop
    }

    fn is_valid_b(&self) -> bool {
        let first = self.password.chars().nth(self.start - 1).unwrap() == self.char;
        let second = self.password.chars().nth(self.stop - 1).unwrap() == self.char;

        first ^ second
    }
}

fn parse_lines(data: &'static str) -> impl Iterator<Item = Password> {
    data.lines().filter_map(|line| line.parse().ok())
}

fn solve_a(data: &'static str) -> Solution {
    parse_lines(data).filter(Password::is_valid).count()
}

fn solve_b(data: &'static str) -> Solution {
    parse_lines(data).filter(Password::is_valid_b).count()
}

fn main() {
    println!("Part 1: {}", solve_a(INPUT));
    println!("Part 2: {}", solve_b(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn examples_a() {
        assert_eq!(solve_a(EXAMPLE), 2);
    }

    #[test]
    fn examples_b() {
        assert_eq!(solve_b(EXAMPLE), 1);
    }
}
