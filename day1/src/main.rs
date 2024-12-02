use std::{collections::HashMap, fs};
use anyhow::Result;
use itertools::Itertools;

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut input: (Vec<&str>, Vec<&str>) = input
        .split_terminator('\n')
        .map(|x| x.split_whitespace().collect_tuple::<(&str, &str)>().unwrap())
        .unzip::<&str, &str, Vec<&str>, Vec<&str>>();
    input.0.sort();
    input.1.sort();
    let input: i32 = input.0
        .iter()
        .zip(input.1.iter())
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .map(|(x, y)| (x-y).abs())
        .sum();
    println!("{}", &input);

    Ok(())
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let input: (Vec<&str>, Vec<&str>) = input
        .split_terminator('\n')
        .map(|x| x.split_whitespace().collect_tuple::<(&str, &str)>().unwrap())
        .unzip::<&str, &str, Vec<&str>, Vec<&str>>();
    let lhs = input.0.iter().map(|x| x.parse::<i32>().unwrap()).collect_vec();
    let rhs = input.1.iter().map(|x| x.parse::<i32>().unwrap()).collect_vec();

    let mut scores: HashMap<i32, i32> = HashMap::new();
    let mut similarity = 0;
    for num in lhs {
        match scores.get(&num) {
            Some(y) => similarity += y,
            None => {
                let score = rhs.iter().filter(|x| **x == num).count() as i32 * num;
                similarity += score;
                scores.insert(num, similarity);
            }
        }
    }

    println!("{}", similarity);

    Ok(())
}

fn main() -> Result<()> {
    part2()?;
    Ok(())
}
