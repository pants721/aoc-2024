use anyhow::Result;
use itertools::Itertools;

fn part1() -> Result<()> {
    let input = include_str!("../input.txt");
    let mut chars = input.chars();

    let mut sum = 0;
    loop {
        if chars.find(|c| *c == 'm').is_none() {
            break;
        }

        if chars.next() != Some('u') 
        || chars.next() != Some('l') 
        || chars.next() != Some('(') {
            continue;
        }

        let lhs = if let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                let mut num = c.to_digit(10).unwrap();
                chars.take_while_ref(|next| next.is_ascii_digit()).for_each(|x| {
                    num *= 10;
                    num += x.to_digit(10).unwrap();
                });
                Some(num)
            } else {
                None
            }
        } else {
            None
        };

        if chars.next() != Some(',') {
            continue;
        }

        let rhs = if let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                let mut num = c.to_digit(10).unwrap();
                chars.take_while_ref(|next| next.is_ascii_digit()).for_each(|x| {
                    num *= 10;
                    num += x.to_digit(10).unwrap();
                });
                Some(num)
            } else {
                None
            }
        } else {
            None
        };

        if lhs.is_some() && rhs.is_some() && chars.next() == Some(')') {
            sum += lhs.unwrap() * rhs.unwrap();
        }
    }
    println!("{}", sum);

    Ok(())
}

fn part2() -> Result<()> {
    let input = include_str!("../input.txt");
    let mut chars = input.chars();

    let mut sum = 0;
    let mut allowed = true;
    loop {
        let first = chars.find(|c| *c == 'm' || *c == 'd');
        if first.is_none() {
            break;
        }

        match first {
            Some('m') => {
                if chars.next() != Some('u') 
                || chars.next() != Some('l') 
                || chars.next() != Some('(') 
                || !allowed {
                    continue;
                }

                let lhs = if let Some(c) = chars.next() {
                    if c.is_ascii_digit() {
                        let mut num = c.to_digit(10).unwrap();
                        chars.take_while_ref(|next| next.is_ascii_digit()).for_each(|x| {
                            num *= 10;
                            num += x.to_digit(10).unwrap();
                        });
                        Some(num)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if chars.next() != Some(',') {
                    continue;
                }

                let rhs = if let Some(c) = chars.next() {
                    if c.is_ascii_digit() {
                        let mut num = c.to_digit(10).unwrap();
                        chars.take_while_ref(|next| next.is_ascii_digit()).for_each(|x| {
                            num *= 10;
                            num += x.to_digit(10).unwrap();
                        });
                        Some(num)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if lhs.is_some() && rhs.is_some() && chars.next() == Some(')') {
                    sum += lhs.unwrap() * rhs.unwrap();
                }
            }
            Some('d') => {
                if chars.next() != Some('o') {
                    continue;
                }

                match chars.next() {
                    Some('(') => {
                        if chars.next() == Some(')') {
                            allowed = true;
                        }
                    },
                    Some('n') => {
                        if chars.next() == Some('\'') 
                        && chars.next() == Some('t')
                        && chars.next() == Some('(')
                        && chars.next() == Some(')') {
                           allowed = false; 
                        }
                    },
                    _ =>  {
                        continue;
                    }
                }
            },
            _ => break,
        }
    }

    println!("{}", sum);

    Ok(())
}

fn main() -> Result<()> {
    part2()?;
    Ok(())
}
