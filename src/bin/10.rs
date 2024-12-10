advent_of_code::solution!(10);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Direction, Location, Map};

use anyhow::{Context, Result};
use std::collections::BTreeSet;

pub fn parse(input: &str) -> Result<Map<u32>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let mut out_line = Vec::new();
        for char in line.chars() {
            out_line.push(char.to_digit(10).context("failed to parse digit")?);
        }
        out.push(out_line);
    }
    Ok(Map(out))
}

pub fn hike_trail(map: &Map<u32>, current: &Location, seen: &mut BTreeSet<Location>) -> u32 {
    let current_value = map.get(current);
    if *current_value == 9 {
        seen.insert(current.clone());
        return 1;
    }
    let mut rating = 0;
    for direction in Direction::all() {
        if let Some(next) = map.go_direction(current, direction) {
            let next_val = map.get(&next);
            if *next_val == current_value + 1 {
                rating += hike_trail(map, &next, seen);
            }
        }
    }
    rating
}
pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let map: Map<u32> = parse(input)?;

    let mut out = 0;
    for row in map.iter() {
        for (loc, v) in row {
            if *v == 0 {
                let mut seen = BTreeSet::new();
                hike_trail(&map, &loc, &mut seen);
                out += seen.len() as u32;
            }
        }
    }
    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let map: Map<u32> = parse(input)?;

    let mut out = 0;
    for row in map.iter() {
        for (loc, v) in row {
            if *v == 0 {
                let mut seen = BTreeSet::new();
                let score = hike_trail(&map, &loc, &mut seen);
                out += score;
            }
        }
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_10 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(36);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(81);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
