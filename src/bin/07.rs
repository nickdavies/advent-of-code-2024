advent_of_code::solution!(7);

use advent_of_code::template::RunType;
use aoc_lib::parse::preamble::*;

use anyhow::{Context, Result};

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
    Con,
}

impl Op {
    fn apply(&self, mut current: u64, mut next: u64) -> u64 {
        match self {
            Self::Add => current + next,
            Self::Mul => current * next,
            Self::Con => {
                while next != 0 {
                    current *= 10;
                    current += next % 10;
                    next /= 10;
                }
                current
            }
        }
    }
}

fn try_ops(remaining: &mut Vec<u64>, current: u64, target: u64, ops: &[Op]) -> bool {
    if current > target {
        return false;
    }
    if let Some(next) = remaining.pop() {
        for op in ops {
            if try_ops(remaining, op.apply(current, next), target, ops) {
                return true;
            }
        }
        remaining.push(next);
        false
    } else {
        current == target
    }
}

fn run_with_ops(input: &str, ops: &[Op]) -> Result<u64> {
    let data: Vec<(u64, Vec<u64>)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, SplitDelim(ParseFromStr, " "), ": "),
        input,
    )
    .context("failed to parse input")?;
    let mut out = 0;
    for (test, mut values) in data {
        values.reverse();

        if try_ops(&mut values, 0, test, ops) {
            out += test;
        }
    }

    Ok(out)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    Ok(Some(run_with_ops(input, &[Op::Add, Op::Mul])?))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    Ok(Some(run_with_ops(input, &[Op::Add, Op::Mul, Op::Con])?))
}

#[cfg(test)]
mod tests_day_7 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(3749);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(11387);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
