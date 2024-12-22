#![allow(unused_imports)]
advent_of_code::solution!(21);

use advent_of_code::template::RunType;

use aoc_lib::grid::{CountingMap, Direction, Location, Map};
use aoc_lib::parse::preamble::*;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

trait ToLocation {
    fn start() -> Self;
    fn void() -> Location;
    fn to_location(&self) -> Location;
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
enum Nums {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    Activate,
}

impl TryFrom<char> for Nums {
    type Error = anyhow::Error;

    fn try_from(other: char) -> Result<Self> {
        Ok(match other {
            '0' => Self::B0,
            '1' => Self::B1,
            '2' => Self::B2,
            '3' => Self::B3,
            '4' => Self::B4,
            '5' => Self::B5,
            '6' => Self::B6,
            '7' => Self::B7,
            '8' => Self::B8,
            '9' => Self::B9,
            'A' => Self::Activate,
            other => {
                return Err(anyhow!("found unexpected nums value {:?}", other));
            }
        })
    }
}

impl ToLocation for Nums {
    fn start() -> Self {
        Self::Activate
    }

    fn void() -> Location {
        Location(3, 0)
    }

    fn to_location(&self) -> Location {
        match self {
            Self::B7 => Location(0, 0),
            Self::B8 => Location(0, 1),
            Self::B9 => Location(0, 2),
            Self::B4 => Location(1, 0),
            Self::B5 => Location(1, 1),
            Self::B6 => Location(1, 2),
            Self::B1 => Location(2, 0),
            Self::B2 => Location(2, 1),
            Self::B3 => Location(2, 2),
            Self::B0 => Location(3, 1),
            Self::Activate => Location(3, 2),
        }
    }
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl TryFrom<char> for Dir {
    type Error = anyhow::Error;

    fn try_from(other: char) -> Result<Self> {
        Ok(match other {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            'A' => Self::Activate,
            other => {
                return Err(anyhow!("found unexpected dir value {:?}", other));
            }
        })
    }
}

impl From<&Dir> for char {
    fn from(other: &Dir) -> Self {
        match other {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
            Dir::Activate => 'A',
        }
    }
}

impl ToLocation for Dir {
    fn start() -> Self {
        Self::Activate
    }

    fn void() -> Location {
        Location(0, 0)
    }

    fn to_location(&self) -> Location {
        match self {
            Self::Up => Location(0, 1),
            Self::Activate => Location(0, 2),
            Self::Left => Location(1, 0),
            Self::Down => Location(1, 1),
            Self::Right => Location(1, 2),
        }
    }
}

fn go_delta_col(delta_col: i32, out: &mut Vec<Dir>) {
    for _ in 0..delta_col.abs() {
        if delta_col < 0 {
            out.push(Dir::Left);
        } else {
            out.push(Dir::Right);
        }
    }
}

fn go_delta_row(delta_row: i32, out: &mut Vec<Dir>) {
    for _ in 0..delta_row.abs() {
        if delta_row < 0 {
            out.push(Dir::Up);
        } else {
            out.push(Dir::Down);
        }
    }
}

fn get_routes<T: ToLocation>(source: &Location, dest: &Location) -> Vec<Vec<Dir>> {
    let void = T::void();
    let delta_row: i32 = dest.0 as i32 - source.0 as i32;
    let delta_col: i32 = dest.1 as i32 - source.1 as i32;

    // If void is on our row and we want to go to void's column we must fix the row first
    if source.0 == void.0 && dest.1 == void.1 {
        let mut out = Vec::new();
        go_delta_row(delta_row, &mut out);
        go_delta_col(delta_col, &mut out);
        return vec![out];
    }

    // If void is on our column and we want to go to void's row we must fix the column first
    if source.1 == void.1 && dest.0 == void.0 {
        let mut out = Vec::new();
        go_delta_col(delta_col, &mut out);
        go_delta_row(delta_row, &mut out);
        return vec![out];
    }
    let mut out_1 = Vec::new();
    go_delta_row(delta_row, &mut out_1);
    go_delta_col(delta_col, &mut out_1);

    let mut out_2 = Vec::new();
    go_delta_col(delta_col, &mut out_2);
    go_delta_row(delta_row, &mut out_2);
    if out_1 == out_2 {
        vec![out_1]
    } else {
        vec![out_1, out_2]
    }
}

fn translate<'a, T, I>(mut current: T, codes: I) -> Vec<Vec<Dir>>
where
    T: ToLocation + Clone + std::fmt::Debug + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut out = Vec::new();
    for next in codes {
        let mut step = Vec::new();
        for mut route in get_routes::<T>(&current.to_location(), &next.to_location()) {
            route.push(Dir::Activate);
            step.push(route);
        }
        current = next.clone();
        out.push(step);
    }
    out.into_iter()
        .multi_cartesian_product()
        .map(|x| x.into_iter().flatten().collect())
        .collect()
}

fn nums_to_int(nums: &[Nums]) -> u64 {
    let mut out = 0;
    for num in nums {
        let n = match num {
            Nums::B0 => 0,
            Nums::B1 => 1,
            Nums::B2 => 2,
            Nums::B3 => 3,
            Nums::B4 => 4,
            Nums::B5 => 5,
            Nums::B6 => 6,
            Nums::B7 => 7,
            Nums::B8 => 8,
            Nums::B9 => 9,
            Nums::Activate => break,
        };
        out *= 10;
        out += n;
    }
    out
}

fn cost_to_go(
    source: Dir,
    dest: Dir,
    level: usize,
    robot_layers: usize,
    cache: &mut BTreeMap<(Location, Location, usize), usize>,
) -> usize {
    // It costs us 1 to press te button
    if level >= robot_layers {
        return 1;
    }

    let key = (source.to_location(), dest.to_location(), level);
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let mut shortest = None;
    for mut route in get_routes::<Dir>(&source.to_location(), &dest.to_location()) {
        route.push(Dir::Activate);
        let mut cost = 0;
        let mut current = Dir::start();
        for step in route {
            cost += cost_to_go(current, step.clone(), level + 1, robot_layers, cache);
            current = step;
        }
        shortest = Some(std::cmp::min(shortest.unwrap_or(cost), cost));
    }

    let cost = shortest.expect("Should always find at least 1 route");
    cache.insert(key, cost);
    cost
}

fn run(input: &str, robot_layers: usize) -> Result<Option<u64>> {
    let data: Vec<Vec<Nums>> =
        parse_input(LineSplitter, Chars(TryFromChar), input).context("failed to parse input")?;

    let mut out: u64 = 0;
    let mut cache = BTreeMap::new();
    for code in &data {
        let mut shortest: Option<usize> = None;
        let code_routes = translate(Nums::start(), code.iter());
        for route in code_routes {
            let mut route_cost = 0;
            let mut current = Dir::start();
            for step in route {
                route_cost += cost_to_go(current, step.clone(), 0, robot_layers, &mut cache);
                current = step;
            }
            shortest = Some(std::cmp::min(shortest.unwrap_or(route_cost), route_cost));
        }

        let int_code = nums_to_int(code);
        let code_cost = shortest.unwrap();
        out += int_code * code_cost as u64;
    }
    Ok(Some(out))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    run(input, 2)
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    run(input, 25)
}

#[cfg(test)]
mod tests_day_21 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(126384);
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
