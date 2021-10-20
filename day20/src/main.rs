#![feature(str_split_once)]
use std::convert::TryInto;

#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Solution = u128;
type Grid = [[bool; 10]; 10];

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Tile {
    id: u128,
    grid: Grid,
}

impl Tile {
    fn new(id: u128, grid: Grid) -> Self {
        Tile { id, grid }
    }

    fn rotate(&self) -> Self {
        let mut grid: Grid = [[]];
        Tile::new(self.id, grid)
    }

    fn flip(&self) -> Self {
        let mut top = self.top.clone();
        top.reverse();
        let mut bottom = self.bottom.clone();
        bottom.reverse();

        Tile {
            id: self.id,
            top,
            right: self.left,
            bottom,
            left: self.right,
        }
    }
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .blocks()
        .map(|block| {
            let mut lines = block.lines();
            let id = &lines
                .next()
                .unwrap()
                .split_once(' ')
                .map_or(0u128, |(_, id)| {
                    u128::from_str_radix(id.trim_end_matches(':'), 10).unwrap()
                });

            let grid: Vec<_> = lines
                .flat_map(|line| line.chars().map(|c| c == '#'))
                .collect();

            Tile::new(
                *id,
                grid.iter()
                    .take(10)
                    .map(|&b| b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                grid.iter()
                    .skip(9)
                    .step_by(10)
                    .map(|&b| b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                grid.iter()
                    .skip(90)
                    .map(|&b| b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                grid.iter()
                    .step_by(10)
                    .map(|&b| b)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect()
}

fn solve_a(tiles: &Vec<Tile>) -> Solution {
    let tiles_lookup: HashMap<_, _> = vec![('a', 'b')].into_iter().collect();

    todo!()
}

fn solve_b(tiles: &Vec<Tile>) -> Solution {
    todo!()
}

fn main() {
    let data = parse_input(INPUT);

    println!("{}", solve_a(&data));
    println!("{}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
}
