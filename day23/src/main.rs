#![feature(str_split_once)]
#[allow(unused_imports)]
use shared::prelude::*;
use std::collections::VecDeque;

const INPUT: &'static str = "614752839";

type Data = VecDeque<u64>;

fn parse_input(input: &str) -> Data {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0) as u64)
        .collect()
}

fn step(data: &mut Data) -> Option<()> {
    let l = data.len() as u64;

    let p = data.pop_front()?;
    let grab = [data.pop_front()?, data.pop_front()?, data.pop_front()?];
    let mut c: u64 = p - 1;
    while grab.contains(&c) || c == 0 {
        if c == 0 {
            c += l + 1;
        }
        c = c - 1;
    }
    let idx = data.iter().position(|&n| n == c)?;
    data.rotate_left(idx + 1);
    data.push_front(grab[2]);
    data.push_front(grab[1]);
    data.push_front(grab[0]);
    data.rotate_right(idx + 1);
    data.push_back(p);

    Some(())
}

fn to_int(data: &Data) -> u64 {
    let mut data = data.clone();
    let idx = data.iter().position(|&n| n == 1).unwrap();
    data.rotate_left(idx);

    data.iter().skip(1).fold(0u64, |acc, &n| acc * 10 + n)
}

fn solve_a(mut data: Data) -> u64 {
    for _ in 0..100 {
        step(&mut data);
    }

    to_int(&data)
}

fn solve_b(mut data: Data) -> u64 {
    data.extend(10..=1000000);
    for _ in 0..10_000_000 {
        step(&mut data);
    }

    let idx = data.iter().position(|&n| n == 1).unwrap();
    data.rotate_left(idx + 1);

    data.pop_front().unwrap() * data.pop_front().unwrap()
}

fn main() {
    let data = parse_input(INPUT);
    println!("{:?}", solve_a(data.clone()));
    println!("{:?}", solve_b(data.clone()));
}
