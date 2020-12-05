const INPUT: &'static str = include_str!("./input.txt");

type Solution = u32;

fn parse_line(line: &str) -> Option<u32> {
    let binary_string: String = line
        .chars()
        .map(|c| match c {
            'B' | 'R' => '1',
            _ => '0',
        })
        .collect();

    u32::from_str_radix(&binary_string, 2).ok()
}

fn parse_input(data: &str) -> Vec<u32> {
    data.lines().filter_map(parse_line).collect()
}

fn find_gap(vec: &Vec<u32>) -> Option<u32> {
    let min = *vec.iter().min()?;
    let max = *vec.iter().max()?;
    let sum: u32 = vec.iter().sum();
    let total_sum = (max * max - min * min + min + max) / 2;

    Some(total_sum - sum)
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
