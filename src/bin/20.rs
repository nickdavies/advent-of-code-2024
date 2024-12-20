advent_of_code::solution!(20);

use advent_of_code::template::RunType;
use aoc_lib::grid::{Direction, Location, Map};

use anyhow::{anyhow, Context, Result};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn shortest_path(
    map: &Map<Option<usize>>,
    current: &Location,
    end: &Location,
    shortest: &mut Vec<Location>,
) -> Result<bool> {
    shortest.push(current.clone());
    if current == end {
        return Ok(true);
    }
    let cost = map
        .get(current)
        .context(format!("Expected to find: {:?}", current))?;

    for direction in Direction::all() {
        if let Some(next) = map.go_direction(current, direction) {
            if let Some(next_cost) = map.get(&next) {
                if *next_cost == cost - 1 {
                    return shortest_path(map, &next, end, shortest);
                }
            }
        }
    }

    unreachable!();
}

fn seek(map: &Map<bool>, start: &Location, end: &Location) -> Map<Option<usize>> {
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

    seen
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

impl Tile {
    fn can_enter(&self) -> bool {
        match self {
            Self::Start => true,
            Self::End => true,
            Self::Empty => true,
            Self::Wall => false,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(other: char) -> Result<Self> {
        Ok(match other {
            'S' => Tile::Start,
            'E' => Tile::End,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            other => {
                return Err(anyhow!("found unexpected tile value {:?}", other));
            }
        })
    }
}

fn find_cheats(locations: &[Location], save_target: usize, cheat_dist: usize) -> usize {
    let mut cheats = 0;
    for (start_idx, start) in locations.iter().enumerate() {
        for (end_idx, end) in locations.iter().enumerate().skip(start_idx + save_target) {
            let noclip = start.manhattan_dist(end);
            let normal = end_idx - start_idx;
            if noclip >= normal {
                continue;
            }
            let saved = normal - noclip;
            if noclip <= cheat_dist && saved >= save_target {
                cheats += 1;
            }
        }
    }
    cheats
}

fn parse(input: &str) -> Result<Vec<Location>> {
    let map: Map<Tile> = Map::try_from(input).context("Failed to parse input")?;
    let start = map
        .find(|(_, t)| *t == &Tile::Start)
        .context("failed to find start")?;
    let end = map
        .find(|(_, t)| *t == &Tile::End)
        .context("failed to find end")?;
    let map: Map<bool> = map.transform(|_, c| !c.can_enter());

    let cost_map = seek(&map, &start, &end);
    let mut shortest = Vec::new();
    shortest_path(&cost_map, &start, &end, &mut shortest)
        .context("expected to find route to exist")?;
    Ok(shortest)
}

pub fn part_one(input: &str, run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let locations = parse(input)?;
    let save_target = match run_type {
        RunType::Real => 100,
        RunType::Example => 12,
    };

    let cheats = find_cheats(&locations, save_target, 2);
    Ok(Some(cheats))
}

pub fn part_two(input: &str, run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let locations = parse(input)?;
    let save_target = match run_type {
        RunType::Real => 100,
        RunType::Example => 50,
    };

    let cheats = find_cheats(&locations, save_target, 20);
    Ok(Some(cheats))
}

#[cfg(test)]
mod tests_day_20 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(3 + 1 + 1 + 1 + 1 + 1);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
