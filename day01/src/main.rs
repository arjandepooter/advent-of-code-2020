const INPUT: &'static str = include_str!("./input.txt");

type Solution = i32;

fn parse_input(data: &str) -> Vec<i32> {
    data.lines().filter_map(|line| line.parse().ok()).collect()
}

fn find_sum(vec: &Vec<i32>, n: i32) -> Option<(i32, i32)> {
    vec.into_iter()
        .find(|entry| vec.contains(&(n - *entry)))
        .map(|entry| (*entry, n - entry))
}

fn find_triplet(vec: &Vec<i32>, n: i32) -> Option<(i32, i32, i32)> {
    (0..vec.len()).find_map(|idx| {
        let mut cloned = vec.clone();
        let entry = cloned.swap_remove(idx);

        find_sum(&cloned, n - entry).map(|(a, b)| (a, b, entry))
    })
}

fn solve_a(data: &str) -> Solution {
    let entries = parse_input(data);
    let (a, b) = find_sum(&entries, 2020).unwrap();

    a * b
}

fn solve_b(data: &str) -> Solution {
    let entries = parse_input(data);
    let (a, b, c) = find_triplet(&entries, 2020).unwrap();

    a * b * c
}

fn main() {
    println!("Part 1: {}", solve_a(INPUT));
    println!("Part 2: {}", solve_b(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1721
979
366
299
675
1456";

    #[test]
    fn examples_a() {
        assert_eq!(solve_a(EXAMPLE), 514579);
    }

    #[test]
    fn examples_b() {
        assert_eq!(solve_b(EXAMPLE), 241861950);
    }
}
