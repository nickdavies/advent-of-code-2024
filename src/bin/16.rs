advent_of_code::solution!(16);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Direction, Location, Map};

use anyhow::{anyhow, Context, Result};
use core::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

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

type CostMap = Map<[Option<usize>; 4]>;

fn build_cost_map(map: &Map<Tile>, target: &Location) -> Result<CostMap> {
    let mut cost_to = map.transform(|_, _| [None; 4]);

    let mut to_visit = BinaryHeap::new();
    for direction in Direction::all() {
        if let Some(next) = map.go_direction(target, &direction.invert()) {
            if map.get(&next).can_enter() {
                to_visit.push(cost_key((0, target.clone(), direction.clone())));
            }
        }
    }
    while !to_visit.is_empty() {
        let (_, (cost, current, from)) = to_visit.pop().unwrap();
        let cache = cost_to.get_mut(&current);
        if let Some(cache_value) = cache[from.index()] {
            if cost >= cache_value {
                continue;
            }
        }
        if let Some(cache_value) = cache[from.invert().index()] {
            if cost + 2000 >= cache_value {
                continue;
            }
        }
        cache[from.index()] = Some(cost);

        for source_direction in [&from, &from.left(), &from.right()] {
            if let Some(next) = map.go_direction(&current, &source_direction.invert()) {
                if map.get(&next).can_enter() {
                    if source_direction == &from {
                        to_visit.push(cost_key((cost + 1, next, source_direction.clone())));
                    } else {
                        to_visit.push(cost_key((cost + 1001, next, source_direction.clone())));
                    }
                }
            }
        }
    }
    Ok(cost_to)
}

fn cost_key(
    data: (usize, Location, Direction),
    // _map: &Map<Tile>,
    // _start: &Location,
    // _target: &Location,
) -> (Reverse<usize>, (usize, Location, Direction)) {
    (Reverse(data.0), data)
}

fn min_cost(cost_map: &CostMap, start: &Location) -> Result<usize> {
    let final_cost = cost_map.get(start);
    let options = vec![
        final_cost[Direction::East.idx()],
        final_cost[Direction::North.idx()].map(|c| c + 1000),
        final_cost[Direction::South.idx()].map(|c| c + 1000),
        final_cost[Direction::West.idx()].map(|c| c + 2000),
    ];

    options
        .into_iter()
        .flatten()
        .min()
        .context("Expected to find a route to target")
}

fn iter_paths(
    cost_map: &CostMap,
    current: &Location,
    target: &Location,
    direction: &Direction,
    allowed_cost: usize,
    seen: &mut BTreeSet<Location>,
) {
    seen.insert(current.clone());
    seen.insert(target.clone());
    if current == target {
        return;
    }

    let costs = cost_map.get(current);
    if costs[direction.idx()].is_some() {
        iter_paths(
            cost_map,
            &cost_map.go_direction(current, direction).unwrap(),
            target,
            direction,
            allowed_cost - 1,
            seen,
        );
    }

    for option in [direction.left(), direction.right()] {
        if let Some(cost) = costs[option.idx()] {
            if cost <= allowed_cost {
                iter_paths(
                    cost_map,
                    current,
                    target,
                    &option,
                    allowed_cost - 1000,
                    seen,
                );
            }
        }
    }
}

fn setup(input: &str) -> Result<(CostMap, Location, Location)> {
    let map: Map<Tile> = input.try_into().context("failed to parse input")?;

    let start = map
        .find(|(_, t)| *t == &Tile::Start)
        .context("failed to find start")?;
    let end = map
        .find(|(_, t)| *t == &Tile::End)
        .context("failed to find end")?;

    let cost_map = build_cost_map(&map, &end).context("Failed to seek path")?;
    Ok((cost_map, start, end))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let (cost_map, start, _) = setup(input)?;
    let min_cost = min_cost(&cost_map, &start)?;

    Ok(Some(min_cost))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let (cost_map, start, end) = setup(input)?;
    let min_cost = min_cost(&cost_map, &start)?;
    let mut locs = BTreeSet::new();
    iter_paths(
        &cost_map,
        &start,
        &end,
        &Direction::East,
        min_cost,
        &mut locs,
    );

    Ok(Some(locs.len()))
}

#[cfg(test)]
mod tests_day_16 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(7036);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(45);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
