#![allow(unused_imports)]
advent_of_code::solution!(19);

use advent_of_code::template::RunType;
use anyhow::{anyhow, Context, Result};
use std::collections::{BTreeMap, BTreeSet};

fn make_towel<'a>(
    target: &'a str,
    options: &[&'a str],
    cache: &mut BTreeMap<&'a str, usize>,
) -> usize {
    if target.is_empty() {
        return 1;
    }

    if let Some(val) = cache.get(target) {
        return *val;
    }
    let mut out = 0;
    for option in options {
        if let Some(after) = target.strip_prefix(option) {
            out += make_towel(after, options, cache);
        }
    }

    cache.insert(target, out);

    out
}

fn run<F: Fn(usize) -> usize>(input: &str, agg_fn: F) -> Result<Option<usize>> {
    let mut lines = input.lines();

    let towels: Vec<&str> = lines
        .next()
        .context("expected a first line")?
        .split(", ")
        .collect();
    let mut patterns = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        patterns.push(line);
    }

    let mut cache = BTreeMap::new();
    let mut possible = 0;
    for pattern in patterns {
        possible += agg_fn(make_towel(pattern, &towels, &mut cache));
    }
    Ok(Some(possible))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    run(input, |possible| if possible == 0 { 0 } else { 1 })
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    run(input, |possible| possible)
}

#[cfg(test)]
mod tests_day_19 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(6);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(16);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
