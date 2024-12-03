use anyhow::Context;
use std::collections::BTreeMap;

advent_of_code::solution!(1);

pub fn part_one(
    input: &str,
    _runtype: advent_of_code::template::RunType,
) -> Result<Option<u32>, anyhow::Error> {
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (left, right) = line
            .split_once(char::is_whitespace)
            .context("failed to split line")?;

        l1.push(left.trim().parse().context("failed to parse left")?);
        l2.push(right.trim().parse().context("failed to parse right")?);
    }

    l1.sort();
    l2.sort();

    let mut delta = 0;
    for (left, right) in std::iter::zip(l1.iter(), l2.iter()) {
        if left > right {
            delta += left - right;
        } else {
            delta += right - left;
        }
    }

    Ok(Some(delta))
}

pub fn part_two(
    input: &str,
    _runtype: advent_of_code::template::RunType,
) -> Result<Option<u32>, anyhow::Error> {
    let mut l1: Vec<u32> = Vec::new();
    let mut l2_hist: BTreeMap<u32, u32> = BTreeMap::new();
    for line in input.lines() {
        let (left, right) = line
            .split_once(char::is_whitespace)
            .context("failed to split line")?;

        l1.push(left.trim().parse().context("failed to parse left")?);
        let right: u32 = right.trim().parse().context("failed to parse right")?;
        *l2_hist.entry(right).or_default() += 1;
    }

    let mut diff = 0;
    for left in l1 {
        diff += l2_hist.get(&left).unwrap_or(&0) * left
    }

    Ok(Some(diff))
}

#[cfg(test)]
mod tests_day_1 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input, advent_of_code::template::RunType::Example)?;
        assert_eq!(result, Some(11));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(input, advent_of_code::template::RunType::Example)?;
        assert_eq!(result, Some(31));
        Ok(())
    }
}
