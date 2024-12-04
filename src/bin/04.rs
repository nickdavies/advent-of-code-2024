advent_of_code::solution!(4);

use advent_of_code::template::RunType;
use anyhow::{Context, Result};
use aoc_lib::grid::{Direction, DirectionIterator, Location, Map};

pub fn make_iterators(map: &Map<char>, start: Location) -> Vec<DirectionIterator<char>> {
    let mut out = Vec::new();
    for x_direction in [None, Some(Direction::North), Some(Direction::South)] {
        for y_direction in [None, Some(Direction::East), Some(Direction::West)] {
            out.push(map.iter_direction(start.clone(), x_direction.clone(), y_direction));
        }
    }
    out
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let mut out = 0;
    let map: Map<char> = input.try_into().context("failed to prase input")?;
    for row in map.iter() {
        for (location, value) in row {
            if *value == 'X' {
                for iter in make_iterators(&map, location.clone()) {
                    let items: Vec<(Location, &char)> = iter.take(3).collect();

                    let word: Vec<char> = items.into_iter().map(|(_, c)| *c).collect();
                    if word == ['M', 'A', 'S'] {
                        out += 1;
                    }
                }
            }
        }
    }
    Ok(Some(out))
}

pub fn get_x(map: &Map<char>, location: &Location) -> Option<((char, char), (char, char))> {
    let n = map.go_direction(location, &Direction::North)?;
    let s = map.go_direction(location, &Direction::South)?;

    let ne = map.go_direction(&n, &Direction::East)?;
    let nw = map.go_direction(&n, &Direction::West)?;

    let se = map.go_direction(&s, &Direction::East)?;
    let sw = map.go_direction(&s, &Direction::West)?;

    Some((
        (*map.get(&ne), *map.get(&sw)),
        (*map.get(&se), *map.get(&nw)),
    ))
}

fn is_mas(a: char, b: char) -> bool {
    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let mut out = 0;
    let map: Map<char> = input.try_into().context("failed to prase input")?;
    for row in map.iter() {
        for (location, value) in row {
            if *value == 'A' {
                if let Some(((ne, sw), (se, nw))) = get_x(&map, &location) {
                    if is_mas(ne, sw) && is_mas(se, nw) {
                        out += 1;
                    }
                }
            }
        }
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_4 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, Some(18));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, Some(9));
        Ok(())
    }
}
