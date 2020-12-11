#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data = HashMap<Point, Cell>;
type Solution = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Occupied,
}

#[derive(Debug, Clone, PartialEq)]
struct Seating {
    map: HashMap<Point, Cell>,
    width: usize,
    height: usize,
    neighbour_threshold: usize,
    look_further: bool,
}

impl Seating {
    fn new(map: &HashMap<Point, Cell>, neighbour_threshold: usize, look_further: bool) -> Self {
        let width = map.keys().map(|Point(_, col)| *col).max().unwrap_or(0) + 1;
        let height = map.keys().map(|Point(row, _)| *row).max().unwrap_or(0) + 1;

        Self {
            map: map.clone(),
            width,
            height,
            neighbour_threshold,
            look_further,
        }
    }

    fn count_neighbours(&self, point: Point, look_further: bool) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(dr, dc)| (*dr, *dc) != (0, 0))
            .filter(|(dr, dc)| {
                match point
                    .iter_direction((*dr, *dc), (self.height, self.width))
                    .skip_while(|p| look_further && self.map.get(p) == None)
                    .next()
                    .map(|p| self.map.get(&p))
                {
                    Some(Some(Cell::Occupied)) => true,
                    _ => false,
                }
            })
            .count()
    }

    fn next_state(&self) -> Self {
        let map = self
            .map
            .iter()
            .map(
                |(point, cell)| match (cell, self.count_neighbours(*point, self.look_further)) {
                    (Cell::Occupied, n) if n >= self.neighbour_threshold => (*point, Cell::Empty),
                    (Cell::Empty, 0) => (*point, Cell::Occupied),
                    (cell, _) => (*point, *cell),
                },
            )
            .collect::<HashMap<_, _>>();

        Seating::new(&map, self.neighbour_threshold, self.look_further)
    }

    fn count_occupied(&self) -> usize {
        self.map.values().filter(|c| **c == Cell::Occupied).count()
    }
}

impl IntoIterator for Seating {
    type Item = Seating;

    type IntoIter = SeatingIterator;

    fn into_iter(self) -> Self::IntoIter {
        let next = self.clone().next_state();

        SeatingIterator {
            current: self,
            next,
        }
    }
}

struct SeatingIterator {
    current: Seating,
    next: Seating,
}

impl Iterator for SeatingIterator {
    type Item = Seating;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current != self.next {
            self.current = self.next.clone();
            self.next = self.current.next_state();
            Some(self.next.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn iter_direction(
        &self,
        direction: (isize, isize),
        (max_row, max_col): (usize, usize),
    ) -> PointIterator {
        PointIterator {
            point: *self,
            direction,
            max_col,
            max_row,
        }
    }
}

struct PointIterator {
    point: Point,
    direction: (isize, isize),
    max_row: usize,
    max_col: usize,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next_row = self.point.0 as isize + self.direction.0;
        let next_col = self.point.1 as isize + self.direction.1;

        if next_row < 0
            || next_row >= self.max_row as isize
            || next_col < 0
            || next_col >= self.max_col as isize
        {
            None
        } else {
            self.point = Point(next_row as usize, next_col as usize);

            Some(self.point)
        }
    }
}

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'L')
                .map(move |(col, _)| (Point(row, col), Cell::Empty))
        })
        .collect::<HashMap<_, _>>()
}

fn solve_a(data: &Data) -> Solution {
    let seating = Seating::new(data, 4, false);

    seating
        .into_iter()
        .last()
        .map(|seating| seating.count_occupied())
        .unwrap_or(0)
}

fn solve_b(data: &Data) -> Solution {
    let seating = Seating::new(data, 5, true);

    seating
        .into_iter()
        .last()
        .map(|seating| seating.count_occupied())
        .unwrap_or(0)
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

    const EXAMPLE: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_a(&data), 37);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_b(&data), 26);
    }
}
