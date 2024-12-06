advent_of_code::solution!(6);

use advent_of_code::template::RunType;

use anyhow::{anyhow, Context, Result};
use aoc_lib::grid::{Direction, Location, Map};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Guard,
    Empty,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '.' => Tile::Empty,
            '^' => Tile::Guard,
            '#' => Tile::Wall,
            other => {
                return Err(anyhow!("unexpected char: {}", other));
            }
        })
    }
}

fn find_guard(map: &Map<Tile>) -> Result<(Location, Direction)> {
    let start_loc = map
        .iter()
        .flatten()
        .filter_map(|(l, t)| if t == &Tile::Guard { Some(l) } else { None })
        .next()
        .context("failed to find guard position")?;

    Ok((start_loc, Direction::North))
}

fn walk_guard(
    map: &Map<Tile>,
    mut current_loc: Location,
    mut current_dir: Direction,
) -> (u32, bool, Map<(bool, [bool; 4])>) {
    let mut seen = map.transform(|_, _| (false, [false; 4]));

    let mut out = 0;
    loop {
        let e = seen.get_mut(&current_loc);
        if !e.0 {
            out += 1;
        }
        e.0 = true;
        if e.1[current_dir.index()] {
            return (out, true, seen);
        }
        e.1[current_dir.index()] = true;

        let new = match map.go_direction(&current_loc, &current_dir) {
            Some(loc) => match map.get(&loc) {
                Tile::Guard | Tile::Empty => (loc, current_dir),
                Tile::Wall => (current_loc, current_dir.right()),
            },
            None => {
                return (out, false, seen);
            }
        };
        current_loc = new.0;
        current_dir = new.1;
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let map: Map<Tile> = input.try_into().context("failed to build map")?;

    let (current_loc, current_dir) = find_guard(&map)?;
    let (out, _, _) = walk_guard(&map, current_loc, current_dir);
    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let mut map: Map<Tile> = input.try_into().context("failed to build map")?;

    let (current_loc, current_dir) = find_guard(&map)?;
    let (_, _, walked_grid) = walk_guard(&map, current_loc.clone(), current_dir.clone());

    let mut out = 0;
    let mut obs = map.transform(|_, _| false);
    for row in walked_grid.iter() {
        for (location, (walked, directions)) in row {
            if !walked {
                continue;
            }
            for (i, valid) in directions.iter().enumerate() {
                if !valid {
                    continue;
                }
                // Can't put one in front of the guard now
                if location == current_loc && i == current_dir.index() {
                    continue;
                }
                let direction = Direction::from_index(i).expect("got unknown index!");
                if let Some(next) = map.go_direction(&location, &direction) {
                    if next == current_loc {
                        continue;
                    }
                    let e = obs.get_mut(&next);
                    if *e {
                        continue;
                    }
                    *e = true;
                    let next_value = map.get_mut(&next);
                    if let Tile::Wall = next_value {
                        continue;
                    }
                    let save = next_value.clone();
                    *next_value = Tile::Wall;
                    let (_, is_loop, _) =
                        walk_guard(&map, current_loc.clone(), current_dir.clone());
                    if is_loop {
                        out += 1;
                    }
                    let next_value = map.get_mut(&next);
                    *next_value = save;
                }
            }
        }
    }

    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_6 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, Some(41));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, Some(6));
        Ok(())
    }
}
