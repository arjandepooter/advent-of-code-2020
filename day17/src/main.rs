#![feature(min_const_generics)]
use std::convert::TryInto;

#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data = Vec<Coord<2>>;
type Solution = usize;
type Coord<const DIMENSIONS: usize> = [isize; DIMENSIONS];

fn parse_input(input: &str) -> Vec<Coord<2>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| [x as isize, y as isize])
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Conway<const DIMENSIONS: usize> {
    grid: HashSet<Coord<DIMENSIONS>>,
}

impl<const DIMENSIONS: usize> Conway<DIMENSIONS> {
    fn from_2d<T>(coords: T) -> Self
    where
        T: IntoIterator<Item = Coord<2>>,
    {
        let grid: HashSet<_> = coords
            .into_iter()
            .map(|coord| {
                [coord.to_vec(), vec![0; DIMENSIONS - 2]]
                    .concat()
                    .try_into()
                    .unwrap()
            })
            .collect();

        Self { grid }
    }

    fn iter_neighbours(&self, coord: Coord<DIMENSIONS>) -> impl Iterator<Item = Coord<DIMENSIONS>> {
        (0isize..(3isize.pow(DIMENSIONS as u32)))
            .filter(|n| (3isize.pow(DIMENSIONS as u32)) / 2 != *n)
            .map(move |offset| {
                (0isize..(DIMENSIONS as isize))
                    .map(|idx| {
                        let offset = (offset / 3isize.pow(idx as u32)) % 3;
                        coord[idx as usize] - 1 + offset
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
    }

    fn next_state(&mut self) {
        let existing: HashSet<_> = self
            .grid
            .iter()
            .filter(|coord| {
                (2..=3).contains(
                    &self
                        .iter_neighbours(**coord)
                        .filter(|neighbour| self.grid.contains(neighbour))
                        .count(),
                )
            })
            .map(|coord| *coord)
            .collect();

        let new_active: HashSet<_> = self
            .grid
            .iter()
            .flat_map(|coord| self.iter_neighbours(*coord))
            .filter(|coord| !self.grid.contains(coord))
            .filter(|coord| {
                self.iter_neighbours(*coord)
                    .filter(|neighbour| self.grid.contains(neighbour))
                    .count()
                    == 3
            })
            .collect();

        self.grid = &existing | &new_active;
    }

    fn run(&mut self, number_of_cycles: usize) {
        for _ in 0..number_of_cycles {
            self.next_state()
        }
    }

    fn number_of_active(&self) -> usize {
        self.grid.len()
    }
}

fn solve_a(data: &Data) -> Solution {
    let mut conway: Conway<3> = Conway::from_2d(data.clone());

    conway.run(6);
    conway.number_of_active()
}

fn solve_b(data: &Data) -> Solution {
    let mut conway: Conway<4> = Conway::from_2d(data.clone());

    conway.run(6);
    conway.number_of_active()
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = ".#.
..#
###";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_a(&data), 112);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_b(&data), 848);
    }
}
