advent_of_code::solution!(15);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Direction, Location, Map};

use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;

enum InputElement {
    Empty,
    Wall,
    Robot,
    CrateWhole,
    CrateLeft,
    CrateRight,
}

impl TryFrom<char> for InputElement {
    type Error = anyhow::Error;

    fn try_from(other: char) -> Result<Self> {
        Ok(match other {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::CrateWhole,
            '[' => Self::CrateLeft,
            ']' => Self::CrateRight,
            unknown => {
                return Err(anyhow!("Hot unexpected char '{}'", unknown));
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Element {
    Empty,
    Wall,
    CrateWhole,
    CrateLeft,
    CrateRight,
}

impl From<&Element> for char {
    fn from(other: &Element) -> char {
        match other {
            Element::Empty => '.',
            Element::Wall => '#',
            Element::CrateWhole => 'O',
            Element::CrateLeft => '[',
            Element::CrateRight => ']',
        }
    }
}

fn calc_gps(map: &Map<Element>) -> usize {
    let mut out = 0;
    for row in map.iter() {
        for (loc, v) in row {
            match v {
                Element::CrateWhole | Element::CrateLeft => out += loc.0 * 100 + loc.1,
                _ => {}
            }
        }
    }
    out
}

fn parse(
    map_input: &str,
    directions_input: &str,
) -> Result<(Location, Map<Element>, Vec<Direction>)> {
    let map: Map<InputElement> = map_input.try_into().context("failed to parse map")?;
    let robot = map
        .iter()
        .flatten()
        .find(|(_, c)| matches!(c, InputElement::Robot))
        .context("Expected to find robot!")?
        .0;

    let map: Map<Element> = map.transform(|_, e| match e {
        InputElement::Empty => Element::Empty,
        InputElement::Wall => Element::Wall,
        InputElement::CrateWhole => Element::CrateWhole,
        InputElement::CrateLeft => Element::CrateLeft,
        InputElement::CrateRight => Element::CrateRight,
        InputElement::Robot => Element::Empty,
    });

    let directions: Vec<Direction> = directions_input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '<' => Ok(Direction::West),
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            other => Err(anyhow!("got unknown direction '{:?}'", other)),
        })
        .collect::<Result<Vec<Direction>>>()
        .context("failed to parse directions")?;

    Ok((robot, map, directions))
}

fn push_double(map: &mut Map<Element>, robot: &Location, direction: &Direction) -> bool {
    assert!(map.get(robot) == &Element::Empty);
    let mut to_move = Vec::new();
    let mut pushing_locations = BTreeSet::new();
    pushing_locations.insert(robot.clone());
    while !pushing_locations.is_empty() {
        let mut next_pushing_locations = BTreeSet::new();
        let mut next_to_move = BTreeSet::new();
        for current in pushing_locations {
            if let Some(next) = map.go_direction(&current, direction) {
                let target = match map.get(&next) {
                    // If we hit a wall we can't move anything
                    Element::Wall => {
                        return false;
                    }
                    // If we find an empty this column is done.
                    Element::Empty => continue,

                    // If we find a crate left/right we add the repsective columns to the next
                    // check
                    Element::CrateLeft => match direction {
                        Direction::East | Direction::West => None,
                        Direction::North | Direction::South => Some(
                            map.go_direction(&next, &Direction::East)
                                .expect("expected a right to CrateLeft"),
                        ),
                    },
                    Element::CrateRight => match direction {
                        Direction::East | Direction::West => None,
                        Direction::North | Direction::South => Some(
                            map.go_direction(&next, &Direction::West)
                                .expect("expected a left to CrateRight"),
                        ),
                    },
                    Element::CrateWhole => None,
                };
                next_to_move.insert(next.clone());
                next_pushing_locations.insert(next.clone());
                if let Some(target) = target {
                    next_to_move.insert(target.clone());
                    next_pushing_locations.insert(target.clone());
                }
            } else {
                panic!("Unexpectedly hit wall!");
            }
        }
        pushing_locations = next_pushing_locations;
        for loc in next_to_move {
            to_move.push(loc);
        }
    }

    for source in to_move.into_iter().rev() {
        let dest = map
            .go_direction(&source, direction)
            .expect("Should be able to move in direction");
        *map.get_mut(&dest) = map.get(&source).clone();
        *map.get_mut(&source) = Element::Empty;
    }

    true
}

fn run(map_input: &str, directions_input: &str) -> Result<Option<usize>> {
    let (mut robot, mut map, directions) =
        parse(map_input, directions_input).context("Failed to parse input")?;

    for direction in &directions {
        if push_double(&mut map, &robot, direction) {
            robot = map
                .go_direction(&robot, direction)
                .context("Expected to be able to move after push said it moved")?;
        }
    }
    Ok(Some(calc_gps(&map)))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let (map_input, directions_str) = input
        .split_once("\n\n")
        .context("Failed to split sections")?;

    run(map_input, directions_str)
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let (map_input, directions_str) = input
        .split_once("\n\n")
        .context("Failed to split sections")?;

    let map_input = map_input
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.");

    run(map_input.as_str(), directions_str)
}

#[cfg(test)]
mod tests_day_15 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(10092);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(9021);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
