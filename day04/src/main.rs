#![feature(str_split_once)]
use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("./input.txt");

type Solution = usize;

fn parse_input(data: &str) -> Vec<HashMap<&str, &str>> {
    data.split("\n\n")
        .map(|block| {
            block
                .lines()
                .flat_map(|line| line.split(' ').filter_map(|part| part.split_once(':')))
                .collect()
        })
        .collect()
}

fn contains_needed_keys(id: &HashMap<&str, &str>) -> bool {
    let needed_keys: HashSet<&str> = (vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
        .into_iter()
        .collect();
    let keys: HashSet<&str> = id.keys().map(|key| *key).collect();

    needed_keys.is_subset(&keys)
}

fn validate_year(year: &str, min: u16, max: u16) -> bool {
    match year.parse::<u16>() {
        Ok(n) => n >= min && n <= max,
        _ => false,
    }
}

fn validate_height(height: &str) -> bool {
    let (amount, unit) = height.split_at(height.len() - 2);

    match amount.parse::<u16>() {
        Ok(h) => match unit {
            "in" => (59..=76).contains(&h),
            "cm" => (150..=193).contains(&h),
            _ => false,
        },
        _ => false,
    }
}

fn validate_color(color: &str) -> bool {
    match color.split_at(1) {
        ("#", value) => value.chars().all(|c| c.is_ascii_hexdigit()),
        _ => false,
    }
}

fn validate_eye_color(color: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color)
}

fn validate_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_numeric())
}

fn is_valid_id(id: &HashMap<&str, &str>) -> bool {
    let validators: &[(&str, fn(&str) -> bool)] = &[
        ("byr", |year| validate_year(year, 1920, 2002)),
        ("iyr", |year| validate_year(year, 2010, 2020)),
        ("eyr", |year| validate_year(year, 2020, 2030)),
        ("hgt", validate_height),
        ("hcl", validate_color),
        ("ecl", validate_eye_color),
        ("pid", validate_pid),
    ];

    validators.into_iter().all(|(key, f)| {
        let value = id.get(key).unwrap_or(&"");
        f(value)
    })
}

fn solve_a(data: &str) -> Solution {
    let ids = parse_input(data);

    ids.into_iter().filter(contains_needed_keys).count()
}

fn solve_b(data: &str) -> Solution {
    let ids = parse_input(data);

    ids.into_iter()
        .filter(contains_needed_keys)
        .filter(is_valid_id)
        .count()
}

fn main() {
    println!("Part 1: {}", solve_a(INPUT));
    println!("Part 2: {}", solve_b(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const VALIDS: &'static str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    const INVALIDS: &'static str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    #[test]
    fn examples_a() {
        assert_eq!(solve_a(EXAMPLE), 2);
    }

    #[test]
    fn examples_b() {
        assert_eq!(solve_b(VALIDS), 4);
        assert_eq!(solve_b(INVALIDS), 0);
    }
}
