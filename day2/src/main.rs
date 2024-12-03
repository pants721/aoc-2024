use std::fs;
use anyhow::Result;
use itertools::Itertools;

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split_terminator('\n').map(|x| x.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect_vec()).collect_vec();
    let res = input.iter().filter(|v| {
        if !v.is_sorted() && !v.iter().rev().is_sorted() { return false };

        for win in v.windows(2) {
            let diff = (win[0] - win[1]).abs();
            if !(1..=3).contains(&diff) { return false };
        }

        true
    }).count();

    println!("{}", res);

    Ok(())
}

fn diffs_are_ok(v: &Vec<i32>) -> bool {
    for win in v.windows(2) {
        let diff = (win[0] - win[1]).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}

fn sorted_and_diffs(v: &Vec<i32>, recursed: bool) -> bool {
    // def improvable control flow wise but wtv
    if recursed {
        return (v.is_sorted() || v.iter().rev().is_sorted()) && diffs_are_ok(v); 
    }

    if (v.is_sorted() || v.iter().rev().is_sorted()) && diffs_are_ok(v) {
        return true;
    }

    for i in 0..v.len() {
        let mut new = v.clone();
        new.remove(i);
        if sorted_and_diffs(&new, true) {
            return true;
        }
    }

    false
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split_terminator('\n').map(|x| x.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect_vec()).collect_vec();
    let res = input.iter().filter(|v| {
        sorted_and_diffs(v, false)
    }).collect_vec();

    println!("{}", res.len());

    Ok(())
}

fn main() -> Result<()> {
    part2()?;
    Ok(())
}
