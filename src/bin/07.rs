advent_of_code::solution!(7);

use advent_of_code::template::RunType;

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

fn try_ops(
    remaining: &mut Vec<u64>,
    current: u64,
    target: u64,
    path: &mut Vec<Op>,
    ops: &[Op],
) -> bool {
    if current > target {
        return false;
    }
    if let Some(next) = remaining.pop() {
        for op in ops {
            path.push(op.clone());
            if try_ops(remaining, op.apply(current, next), target, path, ops) {
                return true;
            }
            path.pop().unwrap();
        }
        remaining.push(next);
        false
    } else {
        current == target
    }
}

fn run_with_ops(input: &str, ops: &[Op]) -> Result<u64> {
    let mut out = 0;
    for line in input.lines() {
        let (test, values) = line.split_once(":").context("failed to split line")?;
        let test: u64 = test.parse().context("failed to parse test value")?;
        let mut values: Vec<u64> = values
            .trim()
            .split(" ")
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()
            .context("failed to parse values")?;
        values.reverse();

        let mut path = Vec::new();
        if try_ops(&mut values, 0, test, &mut path, ops) {
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
