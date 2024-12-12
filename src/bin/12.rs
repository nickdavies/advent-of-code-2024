advent_of_code::solution!(12);

use advent_of_code::template::RunType;

use aoc_lib::grid::{CountingMap, Direction, Location, Map};

use anyhow::{Context, Result};

fn flood(
    map: &Map<char>,
    current: Location,
    seen: &mut CountingMap,
    out: &mut Vec<Location>,
) -> u32 {
    if seen.get(&current) {
        return 0;
    }
    seen.mark(&current);
    out.push(current.clone());
    let plant = map.get(&current);

    let mut edges = 0;
    for direction in Direction::all() {
        if let Some(next) = map.go_direction(&current, direction) {
            if map.get(&next) == plant {
                edges += flood(map, next, seen, out);
            } else {
                edges += 1;
            }
        } else {
            edges += 1;
        }
    }

    edges
}

fn get_regions(map: &Map<char>) -> Vec<(char, u32, Vec<Location>)> {
    let mut seen: CountingMap = map.into();

    let mut regions = Vec::new();
    for row in map.iter() {
        for (loc, c) in row {
            if !seen.get(&loc) {
                let mut locs = Vec::new();
                let perim = flood(map, loc, &mut seen, &mut locs);
                regions.push((*c, perim, locs));
            }
        }
    }
    regions
}

fn is_edge(map: &Map<char>, location: &Location, direction: &Direction) -> bool {
    if let Some(next) = map.go_direction(location, direction) {
        map.get(&next) != map.get(location)
    } else {
        true
    }
}

fn find_side(
    map: &Map<char>,
    current: &Location,
    face_direction: &Direction,
    seen: &mut Map<[bool; 4]>,
) {
    if seen.get(current)[face_direction.idx()] {
        return;
    }
    seen.get_mut(current)[face_direction.idx()] = true;
    let plant = map.get(current);

    // An edge continues left and right of our current location from the edge direction
    // and is only valid if the left/right square is within the region and also that the
    // node in the face_direction is either empty or not the same plant
    for direction in [face_direction.left(), face_direction.right()] {
        if let Some(neighbour) = map.go_direction(current, &direction) {
            if map.get(&neighbour) == plant {
                if let Some(outside) = map.go_direction(&neighbour, face_direction) {
                    // If we are running against the edge of some other plot
                    if map.get(&outside) != plant {
                        find_side(map, &neighbour, face_direction, seen);
                    }
                } else {
                    // if we are running along the map edge
                    find_side(map, &neighbour, face_direction, seen);
                }
            }
        }
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let map: Map<char> = input.try_into().context("failed to parse input")?;

    let regions = get_regions(&map);
    let mut out = 0;
    for (_, p, locs) in &regions {
        out += p * locs.len() as u32;
    }
    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let map: Map<char> = input.try_into().context("failed to parse input")?;

    let regions = get_regions(&map);
    let mut seen = map.transform(|_, _| [false; 4]);
    let mut out = 0;
    for (_, _, locations) in regions {
        let mut sides = 0;
        for direction in Direction::all() {
            for location in &locations {
                if seen.get(location)[direction.index()] {
                    continue;
                }
                if is_edge(&map, location, direction) {
                    sides += 1;
                    find_side(&map, location, direction, &mut seen)
                }
            }
        }
        out += sides * locations.len() as u32;
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_12 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(1930);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(1206);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
