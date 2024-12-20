advent_of_code::solution!(20);

use advent_of_code::template::RunType;
use aoc_lib::grid::{Direction, Location, Map};

use anyhow::{anyhow, Context, Result};

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

fn build_path(map: &Map<Tile>) -> Result<Vec<Location>> {
    let start = map
        .find(|(_, t)| *t == &Tile::Start)
        .context("failed to find start")?;

    let mut path = Vec::new();
    let mut prev = None;
    let mut current = start.clone();
    loop {
        path.push(current.clone());
        if let Tile::End = map.get(&current) {
            break;
        }
        for direction in Direction::all() {
            if let Some(next) = map.go_direction(&current, direction) {
                if Some(&next) == prev.as_ref() {
                    continue;
                }

                if !map.get(&next).can_enter() {
                    continue;
                }

                prev = Some(current);
                current = next;
                break;
            }
        }
    }
    Ok(path)
}

fn find_cheats(path: &[Location], save_target: usize, cheat_dist: usize) -> usize {
    let mut cheats = 0;
    for (start_idx, start) in path.iter().enumerate() {
        for (end_idx, end) in path.iter().enumerate().skip(start_idx + save_target) {
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

pub fn part_one(input: &str, run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let map: Map<Tile> = Map::try_from(input).context("Failed to parse input")?;
    let path = build_path(&map).context("failed to buidl path")?;
    let save_target = match run_type {
        RunType::Real => 100,
        RunType::Example => 12,
    };

    let cheats = find_cheats(&path, save_target, 2);
    Ok(Some(cheats))
}

pub fn part_two(input: &str, run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let map: Map<Tile> = Map::try_from(input).context("Failed to parse input")?;
    let path = build_path(&map).context("failed to buidl path")?;
    let save_target = match run_type {
        RunType::Real => 100,
        RunType::Example => 50,
    };

    let cheats = find_cheats(&path, save_target, 20);
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
