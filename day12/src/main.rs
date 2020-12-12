#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Position = (isize, isize);
type Direction = (isize, isize);
type Command = (char, isize);
type Data = Vec<Command>;
type Solution = usize;

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let (action, value) = line.split_at(1);
            let action = action.chars().next().unwrap();
            let value = value.parse().unwrap_or(0);

            (action, value)
        })
        .collect()
}

fn manhattan_distance((x, y): Position) -> usize {
    (x.abs() + y.abs()) as usize
}

fn rotate((x, y): Direction, angle: isize) -> Direction {
    match angle {
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => (x, y),
    }
}

fn solve_a(data: &Data) -> Solution {
    let (_, position) = data.iter().fold(
        ((1, 0), (0, 0)),
        |((dx, dy), (x, y)), (action, value)| match action {
            'F' => ((dx, dy), (x + dx * value, y + dy * value)),
            'N' => ((dx, dy), (x, y + value)),
            'S' => ((dx, dy), (x, y - value)),
            'E' => ((dx, dy), (x + value, y)),
            'W' => ((dx, dy), (x - value, y)),
            'R' => (rotate((dx, dy), *value), (x, y)),
            'L' => (rotate((dx, dy), 360 - value), (x, y)),
            _ => ((dx, dy), (x, y)),
        },
    );

    manhattan_distance(position)
}

fn solve_b(data: &Data) -> Solution {
    let (_, position) = data.iter().fold(
        ((10, 1), (0, 0)),
        |((dx, dy), (x, y)), (action, value)| match action {
            'F' => ((dx, dy), (x + dx * value, y + dy * value)),
            'N' => ((dx, dy + value), (x, y)),
            'S' => ((dx, dy - value), (x, y)),
            'E' => ((dx + value, dy), (x, y)),
            'W' => ((dx - value, dy), (x, y)),
            'R' => (rotate((dx, dy), *value), (x, y)),
            'L' => (rotate((dx, dy), 360 - value), (x, y)),
            _ => ((dx, dy), (x, y)),
        },
    );

    manhattan_distance(position)
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const EXAMPLE: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_a(&data), 25);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_b(&data), 286);
    }
}
