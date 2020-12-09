#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = Vec<i128>;
type Solution = i128;

fn parse_input(input: &str) -> Data {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn contains_sum(lst: &Vec<i128>, needle: i128) -> bool {
    let mut opposites = Vec::with_capacity(lst.len());

    for n in lst {
        let opposite = needle - *n;

        if opposites.contains(&opposite) {
            return true;
        }
        opposites.push(*n);
    }

    false
}

fn find_invalid_number(data: &Vec<i128>, preamble_length: usize) -> i128 {
    data.into_iter()
        .enumerate()
        .skip(preamble_length)
        .find(|(idx, needle)| {
            !contains_sum(&(data[idx - preamble_length..*idx]).to_vec(), **needle)
        })
        .map(|(_, value)| *value)
        .unwrap_or(0)
}

fn solve_a(data: &Data) -> Solution {
    find_invalid_number(data, 25)
}

fn find_sum_set(data: &Vec<i128>, sum: i128) -> &[i128] {
    let mut front: usize = 0;
    let mut back: usize = 1;

    loop {
        let slc = &data[front..=back];
        let slc_sum: i128 = slc.iter().sum();

        if slc_sum == sum {
            return slc;
        }
        if slc_sum > sum {
            front += 1;
        } else {
            back += 1;
        }
    }
}

fn solve_b(data: &Data) -> Solution {
    let invalid_number = find_invalid_number(data, 25);
    let set = find_sum_set(data, invalid_number);

    set.iter().max().unwrap() + set.iter().min().unwrap()
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

    const EXAMPLE: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);
        assert_eq!(find_invalid_number(&data, 5), 127);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);
        let n = find_invalid_number(&data, 5);
        assert_eq!(find_sum_set(&data, n), &[15, 25, 47, 40]);
    }
}
