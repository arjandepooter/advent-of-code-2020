use std::str::FromStr;

const INPUT: &'static str = include_str!("./input.txt");

type Solution = usize;

#[derive(Copy, Clone)]
enum Cell {
    Tree,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Tree,
            _ => Cell::Empty,
        }
    }
}

impl Cell {
    fn is_tree(&self) -> bool {
        match self {
            Cell::Tree => true,
            _ => false,
        }
    }
}
struct Map(Vec<Vec<Cell>>);

#[derive(Debug)]
struct ParseError;

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<Cell>> = data
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        Ok(Map(map))
    }
}

impl Map {
    fn traverse(&self, right: usize, down: usize) -> Vec<Cell> {
        let map = &self.0;
        let height = map.len() / down;
        let width = map[0].len();

        (0..height)
            .map(|i| map[i * down][i * right % width])
            .collect()
    }
}

fn count_trees(path: Vec<Cell>) -> usize {
    path.into_iter().filter(|cell| cell.is_tree()).count()
}

fn solve_a(data: &str) -> Solution {
    let map: Map = data.parse().unwrap();

    count_trees(map.traverse(3, 1))
}

fn solve_b(data: &str) -> Solution {
    let map: Map = data.parse().unwrap();
    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .into_iter()
        .map(|(right, down)| map.traverse(*right, *down))
        .map(count_trees)
        .product()
}

fn main() {
    println!("Part 1: {}", solve_a(INPUT));
    println!("Part 2: {}", solve_b(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn examples_a() {
        assert_eq!(solve_a(EXAMPLE), 7);
    }

    #[test]
    fn examples_b() {
        assert_eq!(solve_b(EXAMPLE), 336);
    }
}
