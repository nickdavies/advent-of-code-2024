advent_of_code::solution!(3);

use advent_of_code::template::RunType;
use anyhow::{Context, Result};

pub enum Instruction {
    Mult(i32, i32),
    Do,
    Dont,
}

impl Instruction {
    fn from_input(input: &str) -> Option<Self> {
        if input.starts_with("do()") {
            Some(Self::Do)
        } else if input.starts_with("don't()") {
            Some(Self::Dont)
        } else {
            let mut chars = input.chars();
            nom(&mut chars, 'm')?;
            nom(&mut chars, 'u')?;
            nom(&mut chars, 'l')?;
            nom(&mut chars, '(')?;
            Some(Instruction::Mult(
                eat_num(&mut chars, ',')?,
                eat_num(&mut chars, ')')?,
            ))
        }
    }
}

fn eat_num(chars: &mut std::str::Chars, end: char) -> Option<i32> {
    let mut num = String::new();
    let mut next = chars.next()?;
    while next.is_ascii_digit() {
        num.push(next);
        next = chars.next()?;
    }
    if next != end {
        return None;
    }
    Some(num.parse().unwrap())
}

fn nom(chars: &mut std::str::Chars, c: char) -> Option<()> {
    if chars.next()? == c {
        Some(())
    } else {
        None
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    let mut out = Vec::new();
    for (i, c) in input.char_indices() {
        if c == 'm' || c == 'd' {
            if let Some(i) = Instruction::from_input(&input[i..]) {
                out.push(i);
            }
        }
    }
    Ok(out)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<i32>, anyhow::Error> {
    let data = parse_input(input).context("failed to prase input")?;
    let mut out = 0;
    for inst in data {
        if let Instruction::Mult(a, b) = inst {
            out += a * b;
        }
    }
    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<i32>, anyhow::Error> {
    let data = parse_input(input).context("failed to prase input")?;
    let mut out = 0;
    let mut enabled = true;
    for inst in data {
        match inst {
            Instruction::Mult(a, b) => {
                if enabled {
                    out += a * b;
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
        }
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_3 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, Some(161));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, Some(48));
        Ok(())
    }
}
