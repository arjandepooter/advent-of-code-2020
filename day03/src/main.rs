const INPUT: &'static str = include_str!("./input.txt");

type Solution = usize;
type Map = Vec<Vec<Cell>>;

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

fn parse_input(data: &str) -> Map {
    data.lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

fn traverse_map(map: &Map, right: usize, down: usize) -> Vec<Cell> {
    let height = map.len() / down;
    let width = map[0].len();

    (0..height)
        .map(|i| map[i * down][i * right % width])
        .collect()
}

fn solve_a(data: &str) -> Solution {
    let map = parse_input(data);

    traverse_map(&map, 3, 1)
        .into_iter()
        .filter(Cell::is_tree)
        .count()
}

fn solve_b(data: &str) -> Solution {
    let map = parse_input(data);
    let slopes: &[(usize, usize); 5] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .into_iter()
        .map(|(right, down)| traverse_map(&map, *right, *down))
        .map(|path| path.into_iter().filter(Cell::is_tree).count())
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
