#![feature(str_split_once)]
#[allow(unused_imports)]
use shared::prelude::*;

const INPUT: &'static str = include_str!("./input.txt");

type RuleSet = HashMap<usize, Vec<Rule>>;
type Data<'a> = (RuleSet, Vec<&'a str>);
type Solution = usize;

#[derive(Debug, Clone)]
enum Rule {
    Pointer(Vec<usize>),
    Match(char),
}

fn parse_rule(line: &str) -> (usize, Vec<Rule>) {
    let (key, rules) = line.split_once(": ").unwrap();
    let key = key.parse().unwrap();
    let rules = rules
        .split(" | ")
        .map(|rule| {
            if rule.starts_with('"') {
                Rule::Match(rule.chars().nth(1).unwrap())
            } else {
                let pointers = rule.split(" ").filter_map(|s| s.parse().ok()).collect();

                Rule::Pointer(pointers)
            }
        })
        .collect();

    (key, rules)
}

fn parse_input(input: &str) -> Data {
    let (rules, messages) = input.split("\n\n").collect_tuple().unwrap();
    let rules = rules.lines().map(parse_rule).collect();
    let messages = messages.lines().collect();

    (rules, messages)
}

fn is_valid<'a>(message: &'a str, rule_set: &RuleSet, rule_idx: usize) -> Option<&'a str> {
    let rules = rule_set.get(&rule_idx).unwrap();

    rules.into_iter().find_map(|rule| match rule {
        Rule::Pointer(pointers) => pointers
            .into_iter()
            .fold(Some(message), |message, pointer| match message {
                Some(s) => is_valid(s, rule_set, *pointer),
                None => None,
            }),
        Rule::Match(c) if message.starts_with(*c) => Some({
            let (_, tail) = message.split_at(1);
            tail
        }),
        _ => None,
    })
}

fn solve_a((rule_set, messages): &Data) -> Solution {
    messages
        .iter()
        .filter(|message| is_valid(**message, rule_set, 0) == Some(""))
        .count()
}

fn solve_b((rule_set, messages): &Data) -> Solution {
    let mut rule_set = rule_set.clone();
    rule_set.insert(8, vec![Rule::Pointer(vec![42]), Rule::Pointer(vec![42, 8])]);
    rule_set.insert(
        11,
        vec![Rule::Pointer(vec![42, 31]), Rule::Pointer(vec![42, 11, 31])],
    );

    messages
        .iter()
        .filter(|message| is_valid(**message, &rule_set, 0) == Some(""))
        .count()
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn examples_a() {}

    #[test]
    fn examples_b() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_b(&data), 12);
    }
}
