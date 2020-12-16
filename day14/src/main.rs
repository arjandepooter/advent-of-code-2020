#![feature(str_split_once)]
#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<(Vec<char>, Vec<(u64, u64)>)>;
type Solution = u64;

fn parse_mask(line: &str) -> Vec<char> {
    line.trim_start_matches("mask = ").chars().rev().collect()
}

fn parse_assignment(line: &str) -> (u64, u64) {
    let (target, value) = line.split_once(" = ").unwrap();
    let target = target
        .trim_start_matches("mem[")
        .trim_end_matches("]")
        .parse()
        .unwrap();
    let value = value.parse().unwrap();

    (target, value)
}

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    let mut current_mask = parse_mask(lines.next().unwrap());
    let mut assignments = vec![];
    let mut result = vec![];

    for line in lines {
        if line.starts_with("mask = ") {
            result.push((current_mask, assignments));
            assignments = vec![];
            current_mask = parse_mask(line);
        } else if line.starts_with("mem[") {
            let assignment = parse_assignment(line);
            assignments.push(assignment);
        }
    }
    result.push((current_mask, assignments));

    result
}

fn apply_mask(mask: &Vec<char>, value: u64) -> u64 {
    mask.iter().enumerate().fold(0, |acc, (idx, c)| match *c {
        '1' => acc | 1 << idx,
        'X' => acc | ((value >> idx) & 1) << idx,
        _ => acc,
    })
}

fn solve_a(data: &Data) -> Solution {
    let mut mem = HashMap::with_capacity(data.iter().map(|(_, ass)| ass.len()).sum());

    for (mask, assignments) in data {
        for (target, value) in assignments {
            mem.insert(*target, apply_mask(mask, *value));
        }
    }

    mem.values().sum()
}

fn apply_memmask(mask: &Vec<char>, target: u64) -> Vec<char> {
    mask.iter()
        .enumerate()
        .map(|(idx, c)| match c {
            '1' => '1',
            '0' => {
                if ((target >> idx) & 1) == 1 {
                    '1'
                } else {
                    '0'
                }
            }
            c => *c,
        })
        .collect()
}

fn has_overlap(source: &Vec<char>, target: &Vec<char>) -> bool {
    !source
        .iter()
        .zip(target.iter())
        .any(|(sc, tc)| *tc != 'X' && *sc != 'X' && *tc != *sc)
}

fn purge_overlap(source: &Vec<char>, target: &Vec<char>) -> Vec<Vec<char>> {
    // find first overlap
    match source
        .iter()
        .zip(target.iter())
        .find_position(|(sc, tc)| **tc == 'X' && **sc != 'X')
    {
        Some((idx, (sc, _))) => {
            let mut first = target.clone();
            first[idx] = if *sc == '1' { '0' } else { '1' };

            let (head, tail) = target.split_at(idx);
            let mut tail = tail.to_vec();
            tail[0] = *sc;
            let mut source_tail = source.clone();
            source_tail.drain(0..idx);

            purge_overlap(&source_tail, &tail)
                .into_iter()
                .map(|tail| head.iter().chain(&tail).map(|c| *c).collect())
                .chain(std::iter::once(first))
                .collect()
        }
        None => vec![],
    }
}

fn solve_b(data: &Data) -> Solution {
    let mut memmasks: Vec<(Vec<char>, u64)> = Vec::new();

    for (mask, assignments) in data {
        for (target, value) in assignments {
            let source_mask = apply_memmask(mask, *target);
            memmasks = memmasks
                .into_iter()
                .flat_map(|(target_mask, value)| {
                    if has_overlap(&source_mask, &target_mask) {
                        purge_overlap(&source_mask, &target_mask)
                            .into_iter()
                            .map(|mask| (mask, value))
                            .collect::<Vec<(Vec<char>, u64)>>()
                    } else {
                        vec![(target_mask, value)]
                    }
                })
                .collect();
            memmasks.push((source_mask, *value))
        }
    }

    memmasks
        .iter()
        .map(|(mask, value)| *value * 2u64.pow(mask.iter().filter(|c| **c == 'X').count() as u32))
        .sum()
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
    fn examples_a() {
        let data = parse_input(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
        );

        assert_eq!(solve_a(&data), 165);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(
            "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        );
        assert_eq!(solve_b(&data), 208);
    }

    #[test]
    fn overlap() {
        let mask1 = vec!['X', 'X', 'X', '1', '1', 'X'];
        let mask2 = vec!['1', '1', '0', 'X', 'X', 'X'];

        let purged_masks = purge_overlap(&mask2, &mask1);

        assert_eq!(
            purged_masks,
            vec![
                vec!['1', '1', '1', '1', '1', 'X',],
                vec!['1', '0', 'X', '1', '1', 'X',],
                vec!['0', 'X', 'X', '1', '1', 'X',],
            ]
        )
    }

    #[test]
    fn fully_covered() {
        let mask1 = vec!['X', 'X', 'X', '1', '1', 'X'];
        let mask2 = vec!['X', 'X', 'X', '1', '1', 'X'];

        let purged_masks = purge_overlap(&mask2, &mask1);

        assert_eq!(purged_masks, Vec::<Vec<char>>::new());
    }

    #[test]
    fn no_overlap() {
        let mask1 = vec!['X', 'X', '0', '1', '1', 'X'];
        let mask2 = vec!['X', 'X', '1', '1', '1', 'X'];

        assert_eq!(has_overlap(&mask2, &mask1), false);
    }
}
