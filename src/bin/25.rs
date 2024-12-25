advent_of_code::solution!(25);
use advent_of_code::template::RunType;

use anyhow::{Context, Result};
use aoc_lib::grid::{Location, Map};

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for blob in input.split("\n\n") {
        let mut map: Map<char> = blob.try_into().context("failed to parse blob")?;
        let mut key = false;
        if map.get(&Location(0, 0)) == &'.' {
            map = map.transform(|_, c| if *c == '#' { '.' } else { '#' });
            key = true;
        }
        let bottom_right = map.bottom_right().context("expected non-empty map")?;
        let mut heights: Vec<usize> = (0..=bottom_right.1).map(|_| 0).collect();
        for row in map.iter() {
            for (loc, col_val) in row {
                if col_val == &'#' {
                    let height_val = if key {
                        bottom_right.0 - 1 - loc.0
                    } else {
                        loc.0
                    };
                    heights[loc.1] = height_val;
                }
            }
        }
        if key {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }

    let mut out = 0;
    for lock in &locks {
        for key in &keys {
            let mut valid = true;
            for (l, k) in key.iter().zip(lock.iter()) {
                if l + k >= 6 {
                    valid = false;
                    break;
                }
            }
            if valid {
                out += 1;
            }
        }
    }

    Ok(Some(out))
}

pub fn part_two(_input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(None)
}

#[cfg(test)]
mod tests_day_25 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(3);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = None;
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
