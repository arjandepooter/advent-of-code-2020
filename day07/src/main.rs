#![feature(str_split_once)]
#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;
type Solution = usize;

fn strip_bag_part<'a>(part: &'a str) -> &'a str {
    part.trim_end_matches('s').trim_end_matches(" bag")
}

fn parse_line<'a>(line: &'a str) -> (&'a str, Vec<(&'a str, usize)>) {
    let (first, second) = line.trim_end_matches('.').split_once(" contain ").unwrap();
    let children: Vec<_> = second
        .split(", ")
        .filter_map(|s| {
            let (s_amount, bag) = s.split_once(' ')?;
            let amount = s_amount.parse::<usize>().ok()?;
            Some((strip_bag_part(bag), amount))
        })
        .collect();
    (first.trim_end_matches(" bags"), children)
}

fn parse_input(input: &str) -> Data {
    input.lines().map(parse_line).collect()
}

fn solve_a(data: &Data) -> Solution {
    let mut lookup: Vec<&str> = vec!["shiny gold"];
    let mut result = HashSet::new();

    while lookup.len() > 0 {
        let needle = lookup.pop().unwrap();
        result.insert(needle);

        let ext: Vec<_> = data
            .into_iter()
            .filter(|(_, children)| children.into_iter().any(|(bag, _)| *bag == needle))
            .map(|(key, _)| *key)
            .collect();

        lookup.extend(ext);
    }

    result.len() - 1
}

fn count_bags(data: &Data, bag: &str) -> Option<usize> {
    let children = data.get(bag)?;

    Some(
        children
            .into_iter()
            .map(|(bag, amount)| amount * (1 + count_bags(data, bag).unwrap_or(0)))
            .sum(),
    )
}

fn solve_b(data: &Data) -> Solution {
    count_bags(data, "shiny gold").unwrap()
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE_2: &'static str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_a(&data), 4);
    }

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_b(&data), 32);

        let data = parse_input(EXAMPLE_2);
        assert_eq!(solve_b(&data), 126);
    }
}
