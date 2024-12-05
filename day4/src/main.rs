use std::fs;

use anyhow::Result;
use itertools::Itertools;

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let letters = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut total = 0;

    for (i, row) in letters.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let needle = ['X', 'M', 'A', 'S'];
            if *col == 'X' {
                // left
                if (1..=3).map(|x| {
                    row.get(j.saturating_sub(x)) == Some(&needle[x])
                }).all(|b| b) {
                    total += 1;   
                }

                // right
                if (1..=3).map(|x| {
                    row.get(j + x) == Some(&needle[x])
                }).all(|b| b) {
                    total += 1;   
                }

                // up
                if (1..=3).map(|x| {
                    if let Some(next_row) = letters.get(i.saturating_sub(x)) {
                        return next_row.get(j) == Some(&needle[x]);
                    }
                    false
                }).all(|b| b) {
                    total += 1;   
                }

                // down
                if (1..=3).map(|x| {
                    if let Some(next_row) = letters.get(i + x) {
                        return next_row.get(j) == Some(&needle[x]);
                    }
                    false
                }).all(|b| b) {
                    total += 1;   
                }

                // up & left
                if (1..=3).map(|x| {
                    if let Some(new_row_idx) = i.checked_sub(x) {
                        if let Some(next_row) = letters.get(new_row_idx) {
                            if let Some(new_col_idx) = j.checked_sub(x) {
                                return next_row.get(new_col_idx) == Some(&needle[x]);
                            }
                        }
                    }
                    false
                }).all(|b| b) {
                    total += 1;   
                }
                
                // up & right
                if (1..=3).map(|x| {
                    if let Some(new_row_idx) = i.checked_sub(x) {
                        if let Some(next_row) = letters.get(new_row_idx) {
                            return next_row.get(j + x) == Some(&needle[x]);
                        }
                    }
                    false
                }).all(|b| b) {
                    total += 1;   
                }

                // down & left
                if (1..=3).map(|x| {
                    if let Some(next_row) = letters.get(i + x) {
                        if let Some(new_col_idx) = j.checked_sub(x) {
                            return next_row.get(new_col_idx) == Some(&needle[x]);
                        }
                    }
                    false
                }).all(|b| b) {
                    total += 1;   
                }

                // down & right
                if (1..=3).map(|x| {
                    if let Some(next_row) = letters.get(i + x) {
                        return next_row.get(j + x) == Some(&needle[x]);
                    }
                    false
                }).all(|b| b) {
                    total += 1;   
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let letters = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut total = 0;

    for (i, row) in letters.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter == 'A' && || -> Option<()> {
                // Top left to bottom right
                let (left, up) = (j.checked_sub(1)?, i.checked_sub(1)?);
                let top_row = letters.get(up)?;
                let tl_letter = top_row.get(left)?;
                let bottom_row = letters.get(i + 1)?;
                let br_letter = bottom_row.get(j + 1)?;
                // Top right to bottom left
                let tr_letter = top_row.get(j + 1)?;
                let bl_letter = bottom_row.get(left)?;
                if ((*tl_letter == 'M' && *br_letter == 'S') 
                || (*tl_letter == 'S' && *br_letter == 'M')) 
                && ((*tr_letter == 'M' && *bl_letter == 'S')
                || (*tr_letter == 'S' && *bl_letter == 'M'))
                {
                    total += 1;
                }
                Some(())
            }().is_none() {
                continue;
            }
        }
    }

    println!("{}", total);
    Ok(())
}

fn main() -> Result<()> {
    part2()?;

    Ok(())
}
