#![feature(str_split_once)]
#[allow(unused_imports)]
use shared::prelude::*;
use std::ops::RangeInclusive;

const INPUT: &'static str = include_str!("./input.txt");

type Data<'a> = TicketCollection<'a>;
type Solution = usize;

fn parse_constraints<'a>(constraints: &'a str) -> HashMap<&'a str, Vec<RangeInclusive<usize>>> {
    constraints
        .lines()
        .filter_map(|line| {
            let (name, ranges) = line.split_once(": ")?;
            let ranges = ranges
                .split(" or ")
                .filter_map(|range| {
                    let (start, stop) = range.split_once("-")?;
                    let start = start.parse().ok()?;
                    let stop = stop.parse().ok()?;

                    Some(start..=stop)
                })
                .collect();
            Some((name, ranges))
        })
        .collect()
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(',').filter_map(|n| n.parse().ok()).collect()
}

fn parse_input<'a>(input: &'a str) -> Data<'a> {
    let (constraints, my_ticket, tickets) = input.split("\n\n").collect_tuple().unwrap();
    let constraints = parse_constraints(constraints);
    let my_ticket = my_ticket.lines().skip(1).map(parse_ticket).next().unwrap();
    let tickets = tickets.lines().skip(1).map(parse_ticket).collect();

    TicketCollection {
        my_ticket,
        tickets,
        constraints,
    }
}

fn transpose<T: IntoIterator, U>(lst: &T) -> Vec<Vec<U>>
where
    <T as IntoIterator>::Item: IntoIterator<Item = U>,
    U: Copy + Sized,
{
    let mut transposed = vec![vec![]; lst.iter().count()];

    for row in lst.into_iter() {
        for (idx, column) in row.into_iter().enumerate() {
            transposed[idx].push(column);
        }
    }

    transposed
}

#[derive(Debug, Clone)]
struct TicketCollection<'a> {
    constraints: HashMap<&'a str, Vec<RangeInclusive<usize>>>,
    my_ticket: Vec<usize>,
    tickets: Vec<Vec<usize>>,
}

impl<'a> TicketCollection<'a> {
    fn verify_ticket(&self, ticket: &Vec<usize>) -> Option<usize> {
        ticket
            .iter()
            .find(|n| {
                self.constraints
                    .iter()
                    .map(|(_, ranges)| ranges)
                    .flatten()
                    .all(|range| !range.contains(n))
            })
            .map(|n| *n)
    }

    fn invalid_tickets(&self) -> Vec<usize> {
        self.tickets
            .iter()
            .filter_map(|ticket| self.verify_ticket(ticket))
            .collect()
    }

    fn valid_tickets(&self) -> Vec<&Vec<usize>> {
        self.tickets
            .iter()
            .filter(|ticket| self.verify_ticket(ticket).is_none())
            .collect()
    }

    fn valid_fields_per_column(&self) -> Vec<Vec<&str>> {
        let columns = transpose(&self.valid_tickets());

        columns
            .into_iter()
            .map(|items| {
                self.constraints
                    .iter()
                    .filter_map(move |(field, ranges)| {
                        if items
                            .iter()
                            .all(|item| ranges.iter().any(|range| range.contains(item)))
                        {
                            Some(*field)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

// fn possible_fields(fields: &Vec<Vec<&str>>, satisfied_fields: &Vec<&str>) -> Option<Vec<&str>> {
//     if fields.len() == 0 {
//         return Some(vec![]);
//     }

//     let x = [
//         ["duration"],
//         ["departure time"],
//         ["departure location"],
//         ["type"],
//         ["train"],
//         ["price"],
//         ["wagon"],
//         ["arrival track"],
//         ["zone"],
//         ["departure track"],
//         ["arrival station"],
//         ["departure date"],
//         ["arrival platform"],
//         ["departure platform"],
//         ["departure station"],
//         ["class"],
//         ["route"],
//         ["seat"],
//         ["arrival location"],
//         ["row"],
//     ];

//     let current = fields[0].clone();
//     let rest: Vec<_> = fields.iter().skip(1).collect();
//     for field in current {}

//     None
// }

fn solve_a(data: &Data) -> Solution {
    data.invalid_tickets().iter().sum()
}

fn solve_b(data: &Data) -> Solution {
    data.valid_fields_per_column().into_debug("FIELDS");
    0
}

fn main() {
    let data = parse_input(INPUT);

    println!("Part A: {}", solve_a(&data));
    println!("Part B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn examples_a() {
        let data = parse_input(EXAMPLE);

        assert_eq!(solve_a(&data), 71);
    }
}
