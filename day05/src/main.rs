const INPUT: &'static str = include_str!("./input.txt");

type Solution = u16;

fn parse_input(data: &str) -> Vec<u16> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'B' | 'R' => '1',
                    _ => '0',
                })
                .collect::<String>()
        })
        .filter_map(|s| u16::from_str_radix(&s, 2).ok())
        .collect()
}

fn find_gap(vec: &Vec<u16>) -> Option<u16> {
    let mut cloned = vec.clone();
    cloned.sort();

    let second_iter = cloned.clone().into_iter().skip(1);

    cloned
        .into_iter()
        .zip(second_iter)
        .find_map(|(a, b)| if b - a == 2 { Some(a + 1) } else { None })
}

fn solve_a(data: &str) -> Solution {
    let passes = parse_input(data);

    passes.into_iter().max().unwrap_or(0)
}

fn solve_b(data: &str) -> Solution {
    let passes = parse_input(data);

    find_gap(&passes).unwrap()
}

fn main() {
    println!("Part 1: {}", solve_a(INPUT));
    println!("Part 2: {}", solve_b(INPUT));
}
