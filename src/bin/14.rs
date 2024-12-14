advent_of_code::solution!(14);

use advent_of_code::template::RunType;

use aoc_lib::grid::{Location, Map};
use aoc_lib::parse::preamble::*;

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn to_location(&self) -> Location {
        Location(self.y as usize, self.x as usize)
    }
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let (_, right) = input.split_once("=").context("failed to split on =")?;

        let (x, y) = right
            .split_once(",")
            .context("failed to splix x/y components")?;

        Ok(Point {
            x: x.parse().context("failed to convert x to int")?,
            y: y.parse().context("failed to convert y to int")?,
        })
    }
}

#[derive(Debug, Clone)]
struct Vel {
    x: i64,
    y: i64,
}

impl std::str::FromStr for Vel {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let (_, right) = input.split_once("=").context("failed to split on =")?;

        let (x, y) = right
            .split_once(",")
            .context("failed to splix x/y components")?;

        Ok(Vel {
            x: x.parse().context("failed to convert x to int")?,
            y: y.parse().context("failed to convert y to int")?,
        })
    }
}

#[derive(Debug, Clone)]
struct Dims {
    width: i64,
    height: i64,
}

#[derive(Debug, Clone)]
struct Robot {
    id: usize,
    point: Point,
    vel: Vel,
    dims: Dims,
}

impl Robot {
    fn step(&mut self) {
        self.point.x = (self.point.x + self.dims.width + self.vel.x) % self.dims.width;
        self.point.y = (self.point.y + self.dims.height + self.vel.y) % self.dims.height;
    }

    fn quadrant(&self) -> Option<usize> {
        let mid_x = self.dims.width / 2;
        let mid_y = self.dims.height / 2;
        match (self.point.x.cmp(&mid_x), self.point.y.cmp(&mid_y)) {
            (_, Ordering::Equal) | (Ordering::Equal, _) => None,
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Less, Ordering::Greater) => Some(1),
            (Ordering::Greater, Ordering::Less) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
        }
    }

    fn cycle(&self) -> RobotCycle {
        let mut robot = self.clone();
        let mut points = Vec::new();

        points.push(robot.point.clone());
        robot.step();
        while robot.point != self.point {
            robot.step();
            points.push(robot.point.clone());
        }
        RobotCycle {
            id: self.id,
            points,
        }
    }
}

#[derive(Debug, Clone)]
struct RobotCycle {
    id: usize,
    points: Vec<Point>,
}

impl RobotCycle {
    fn at_time(&self, time: usize) -> Point {
        self.points[time % self.points.len()].clone()
    }
}

fn row_hist<I: Iterator<Item = i64>>(
    x: i64,
    ys: I,
    locations: &BTreeMap<Point, Vec<(usize, usize, usize)>>,
) -> BTreeMap<usize, usize> {
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for y in ys {
        let p = Point { x, y };
        if let Some(points) = locations.get(&p) {
            for (_, time, _) in points {
                *hist.entry(*time).or_default() += 1;
            }
        }
    }
    hist
}

fn parse(input: &str, dims: &Dims) -> Result<Vec<Robot>> {
    let data: Vec<(Point, Vel)> = parse_input(
        LineSplitter,
        ParseTuple2(ParseFromStr, ParseFromStr, " "),
        input,
    )
    .context("failed to parse input")?;

    let mut robots = Vec::new();

    for (id, (point, vel)) in data.into_iter().enumerate() {
        robots.push(Robot {
            id,
            point,
            vel,
            dims: dims.clone(),
        });
    }

    Ok(robots)
}

pub fn part_one(input: &str, run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let dims = match run_type {
        RunType::Example => Dims {
            width: 11,
            height: 7,
        },
        RunType::Real => Dims {
            width: 101,
            height: 103,
        },
    };

    let mut robots = parse(input, &dims)?;

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.step();
        }
    }

    let mut quads = [0; 4];
    for robot in robots {
        if let Some(idx) = robot.quadrant() {
            quads[idx] += 1;
        }
    }

    Ok(Some(quads[0] * quads[1] * quads[2] * quads[3]))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let dims = Dims {
        width: 101,
        height: 103,
    };
    let robots = parse(input, &dims)?;

    let mut cycles = Vec::new();
    for robot in robots.iter() {
        cycles.push(robot.cycle());
    }

    let mut locations: BTreeMap<Point, Vec<(usize, usize, usize)>> = BTreeMap::new();
    for cycle in cycles.iter() {
        for (step_count, step) in cycle.points.iter().enumerate() {
            locations.entry(step.clone()).or_default().push((
                cycle.id,
                step_count,
                cycle.points.len(),
            ));
        }
    }

    let cols: Vec<i64> = (0..dims.width).collect();
    let window_width = 30;
    let mut options = BTreeSet::new();
    for x in 0..dims.height {
        let hist = row_hist(x, cols.iter().cloned(), &locations);
        let mut check = false;
        for count in hist.values() {
            if *count >= window_width {
                check = true;
                break;
            }
        }
        if check {
            for window in cols.windows(window_width) {
                let hist = row_hist(x, window.iter().cloned(), &locations);
                for (time, count) in hist {
                    if count >= window_width {
                        options.insert(time);
                    }
                }
            }
        }
    }

    for option in options.iter() {
        let mut display_grid =
            Map::from_dimensions(dims.height as usize, dims.width as usize, |_| '.');

        for cycle in cycles.iter() {
            let point = cycle.at_time(*option);
            *display_grid.get_mut(&point.to_location()) = 'X';
        }
        display_grid.print(|c, _| *c);
    }

    if options.len() > 1 {
        Err(anyhow!("Can't go down to a single answer!"))
    } else {
        Ok(Some(options.into_iter().next().unwrap() as u32 + 1))
    }
}

#[cfg(test)]
mod tests_day_14 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(12);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
