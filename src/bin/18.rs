advent_of_code::solution!(18);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Direction, Location, Map, UnboundLocation};
use aoc_lib::parse::preamble::*;

use anyhow::{Context, Result};
use core::cmp::Reverse;
use std::collections::BinaryHeap;

fn seek(map: &Map<bool>, start: &Location, end: &Location) -> Option<usize> {
    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), 0, end.clone()));

    let mut seen = map.transform(|_, _| None);

    while !to_visit.is_empty() {
        let (_, dist, current) = to_visit.pop().unwrap();
        if let Some(existing_dist) = seen.get(&current) {
            if existing_dist <= &dist {
                continue;
            }
        }
        *seen.get_mut(&current) = Some(dist);
        if &current == start {
            break;
        }

        for direction in Direction::all() {
            if let Some(next) = map.go_direction(&current, direction) {
                if !map.get(&next) {
                    to_visit.push((Reverse(dist), dist + 1, next.clone()))
                }
            }
        }
    }

    *seen.get(start)
}

fn parse(input: &str, dim: usize) -> Result<(Vec<Location>, Location, Location)> {
    let data: Vec<(i64, i64)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, ParseFromStr, ","),
        input,
    )
    .context("failed to parse input")?;
    let map = Map::from_dimensions(dim, dim, |_| false);

    let mut blocks = Vec::new();
    for (x, y) in data {
        let loc = UnboundLocation(y, x)
            .to_bounded(&map)
            .context("expected to build real location")?;

        blocks.push(loc);
    }

    let start = UnboundLocation(0, 0)
        .to_bounded(&map)
        .context("expected to build real location")?;
    let end = UnboundLocation(dim as i64 - 1, dim as i64 - 1)
        .to_bounded(&map)
        .context("expected to build real location")?;

    Ok((blocks, start, end))
}

fn mark<'a, I: Iterator<Item = &'a Location>>(map: &'a mut Map<bool>, blocks: I) {
    for loc in blocks {
        *map.get_mut(loc) = true;
    }
}

pub fn part_one(input: &str, run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let (dim, drop) = match run_type {
        RunType::Example => (7, 12),
        RunType::Real => (71, 1024),
    };
    let (blocks, start, end) = parse(input, dim).context("failed to parse input")?;

    let mut map = Map::from_dimensions(dim, dim, |_| false);
    mark(&mut map, blocks.iter().take(drop));

    let dist = seek(&map, &start, &end).context("expected to find route to exist")?;

    Ok(Some(dist))
}

pub fn part_two(input: &str, run_type: RunType) -> Result<Option<String>, anyhow::Error> {
    let dim = match run_type {
        RunType::Example => 7,
        RunType::Real => 71,
    };
    let (blocks, start, end) = parse(input, dim).context("failed to parse input")?;

    let mut left = 0;
    let mut right = blocks.len();
    while left != right && left + 1 != right {
        let mid = left + ((right - left) / 2);
        let mut map = Map::from_dimensions(dim, dim, |_| false);
        mark(&mut map, blocks.iter().take(mid));

        let can_route = seek(&map, &start, &end).is_some();
        if can_route {
            left = mid;
        } else {
            right = mid;
        }
    }

    let block = &blocks[left];

    Ok(Some(format!("{},{}", block.1, block.0)))
}

#[cfg(test)]
mod tests_day_18 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(22);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some("6,1".to_string());
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
