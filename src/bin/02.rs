advent_of_code::solution!(2);

use advent_of_code::template::RunType;
use anyhow::{Context, Result};
use std::cmp::Ordering;

pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let mut report: Vec<i32> = Vec::new();
        for level in line.split_whitespace() {
            report.push(level.parse().context("failed to parse int")?);
        }
        out.push(report);
    }

    Ok(out)
}

pub fn safe_report(report: &[i32]) -> bool {
    let mut dir = None;
    for ab in report.windows(2) {
        if (ab[0] - ab[1]).abs() > 3 || ab[0] == ab[1] {
            return false;
        }
        let delta = ab[0].cmp(&ab[1]);
        match (dir, delta) {
            (None, _) => {
                dir = Some(delta);
            }
            (Some(Ordering::Greater), Ordering::Greater) => continue,
            (Some(Ordering::Less), Ordering::Less) => continue,
            (_, _) => {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data = parse_input(input).context("failed to prase input")?;
    let mut safe_count = 0;
    for report in &data {
        if safe_report(report) {
            safe_count += 1;
        }
    }
    Ok(Some(safe_count))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data = parse_input(input).context("failed to prase input")?;
    let mut safe_count = 0;
    for report in &data {
        if safe_report(report) {
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut sub = Vec::new();
                for (n, v) in report.iter().enumerate() {
                    if i != n {
                        sub.push(*v);
                    }
                }
                if safe_report(&sub) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    Ok(Some(safe_count))
}

#[cfg(test)]
mod tests_day_02 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, Some(2));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, Some(4));
        Ok(())
    }
}
