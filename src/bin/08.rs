advent_of_code::solution!(8);

use advent_of_code::template::RunType;
use anyhow::{Context, Result};
use aoc_lib::grid::{CountingMap, Location, Map, UnboundLocation};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum Ant {
    Empty,
    Ant(char),
}

impl TryFrom<char> for Ant {
    type Error = anyhow::Error;
    fn try_from(other: char) -> Result<Self> {
        Ok(match other {
            '.' => Self::Empty,
            other => Self::Ant(other),
        })
    }
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn make_equal_antis(map: &Map<Ant>, loc_a: &Location, loc_b: &Location, antis: &mut CountingMap) {
    let dx = (loc_a.0 as i64) - (loc_b.0 as i64);
    let dy = (loc_a.1 as i64) - (loc_b.1 as i64);

    if let Some(a) = UnboundLocation(loc_a.0 as i64 + dx, loc_a.1 as i64 + dy).to_bounded(map) {
        antis.mark(&a);
    }
    if let Some(a) = UnboundLocation(loc_b.0 as i64 - dx, loc_b.1 as i64 - dy).to_bounded(map) {
        antis.mark(&a);
    }
}

fn walk_until_invalid(
    map: &Map<Ant>,
    mut current: Location,
    other: &Location,
    dx: i64,
    dy: i64,
    out: &mut CountingMap,
) {
    loop {
        let next = UnboundLocation(current.0 as i64 + dx, current.1 as i64 + dy);
        match next.to_bounded(map) {
            Some(n) => {
                if &n != other {
                    out.mark(&n);
                }
                current = n;
            }
            None => return,
        }
    }
}

fn iterate_all_antis(map: &Map<Ant>, loc_a: &Location, loc_b: &Location, antis: &mut CountingMap) {
    let dx = (loc_a.0 as i64) - (loc_b.0 as i64);
    let dy = (loc_a.1 as i64) - (loc_b.1 as i64);

    let common = gcd_of_two_numbers(dx.unsigned_abs(), dy.unsigned_abs());

    let cdx = dx / common as i64;
    let cdy = dy / common as i64;

    walk_until_invalid(map, loc_a.clone(), loc_b, cdx, cdy, antis);
    walk_until_invalid(map, loc_b.clone(), loc_a, -cdx, -cdy, antis);

    antis.mark(loc_a);
    antis.mark(loc_b);
}

fn run(input: &str, anti_fn: fn(&Map<Ant>, &Location, &Location, &mut CountingMap)) -> Result<u32> {
    let map: Map<Ant> = input.try_into().context("failed to parse input")?;

    let mut ants: BTreeMap<char, Vec<Location>> = BTreeMap::new();
    for row in map.iter() {
        for (loc, value) in row {
            if let Ant::Ant(c) = value {
                ants.entry(*c).or_default().push(loc);
            }
        }
    }

    let mut anti = CountingMap::from(&map);
    for locs in ants.values() {
        for loc_a in locs {
            for loc_b in locs {
                if loc_a == loc_b {
                    continue;
                }
                anti_fn(&map, loc_a, loc_b, &mut anti);
            }
        }
    }
    Ok(anti.unique() as u32)
}
pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(Some(run(input, make_equal_antis)?))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(Some(run(input, iterate_all_antis)?))
}

#[cfg(test)]
mod tests_day_8 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(14);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(34);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two_extra_1() -> anyhow::Result<()> {
        let expected = Some(9);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 3);
        assert!(expected.is_none() || !input.is_empty(), "example 3 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
