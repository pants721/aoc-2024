#![feature(int_roundings)]
use std::{cmp::Ordering, collections::{HashMap, VecDeque}};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Page {
    before: Vec<i32>,
    after: Vec<i32>,
}

fn part1() -> Result<()> {
    let input = include_str!("../input.txt");

    let mut pages: HashMap<i32, Page> = HashMap::new();

    let (first, second) = input.split_at(input.find("\n\n").unwrap());
    let (first, second) = (first.to_string(), second.to_string());

    first.split_terminator(&['\n', '|'][..]).tuples().for_each(|t: (&str, &str)| {
        if let (Ok(lhs), Ok(rhs)) = (t.0.to_string().parse::<i32>(), t.1.to_string().parse::<i32>()) {
            match pages.get_mut(&lhs) {
                Some(page) => {
                    page.after.push(rhs);
                },
                None => {
                    let page = Page {
                        after: vec![rhs],
                        before: vec![],
                    };
                    pages.insert(lhs, page);
                }
            }

            match pages.get_mut(&rhs) {
                Some(page) => {
                    page.before.push(lhs);
                },
                None => {
                    let page = Page {
                        after: vec![],
                        before: vec![lhs],
                    };
                    pages.insert(rhs, page);
                }
            }

        }
    });

    let res: i32 = second
        .split_terminator('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s
            .to_string()
            .split_terminator(',')
            .flat_map(|s| s.to_string().parse::<i32>())
            .collect_vec()
        )
        .filter(|s| s
            .is_sorted_by(|lhs, rhs| {
                pages.get(lhs).unwrap().after.contains(rhs)
                && pages.get(rhs).unwrap().before.contains(lhs)
            })
        )
        .map(|v| v.as_slice()[v.len().div_floor(2)])
        .sum();

    println!("{}", res);

    Ok(())
}

fn part2() -> Result<()> {
    let input = include_str!("../input.txt");

    let mut pages: HashMap<i32, Page> = HashMap::new();

    let (first, second) = input.split_at(input.find("\n\n").unwrap());
    let (first, second) = (first.to_string(), second.to_string());

    first.split_terminator(&['\n', '|'][..]).tuples().for_each(|t: (&str, &str)| {
        if let (Ok(lhs), Ok(rhs)) = (t.0.to_string().parse::<i32>(), t.1.to_string().parse::<i32>()) {
            match pages.get_mut(&lhs) {
                Some(page) => {
                    page.after.push(rhs);
                },
                None => {
                    let page = Page {
                        after: vec![rhs],
                        before: vec![],
                    };
                    pages.insert(lhs, page);
                }
            }

            match pages.get_mut(&rhs) {
                Some(page) => {
                    page.before.push(lhs);
                },
                None => {
                    let page = Page {
                        after: vec![],
                        before: vec![lhs],
                    };
                    pages.insert(rhs, page);
                }
            }

        }
    });

    let res: i32 = second
        .split_terminator('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s
            .to_string()
            .split_terminator(',')
            .flat_map(|s| s.to_string().parse::<i32>())
            .collect_vec()
        )
        .filter(|s| !s
            .is_sorted_by(|lhs, rhs| {
                pages.get(lhs).unwrap().after.contains(rhs)
                && pages.get(rhs).unwrap().before.contains(lhs)
            })
        )
        .map(|mut s| {
            s.sort_by(|lhs, rhs| {
                if pages.get(lhs).unwrap().after.contains(rhs)
                && pages.get(rhs).unwrap().before.contains(lhs) {
                    return Ordering::Less;
                }

                Ordering::Greater
            });
            s
        })
        .map(|v| v.as_slice()[v.len().div_floor(2)])
        .sum();

    println!("{}", res);

    Ok(())
}

fn main() -> Result<()> {
    part2()?;

    Ok(())
}
