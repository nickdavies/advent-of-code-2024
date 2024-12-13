advent_of_code::solution!(13);

use advent_of_code::template::RunType;

use anyhow::{Context, Result};
use aoc_lib::parse::preamble::*;

struct Point {
    x: i64,
    y: i64,
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let (_, right) = input.split_once(": ").context("failed to split on :")?;

        let (x, y) = right
            .split_once(", ")
            .context("failed to splix x/y components")?;

        Ok(Point {
            x: x.split_at_checked(2)
                .context("failed to remove x prefix")?
                .1
                .parse()
                .context("failed to convert x to int")?,
            y: y.split_at_checked(2)
                .context("failed to remove y prefix")?
                .1
                .parse()
                .context("failed to convert y to int")?,
        })
    }
}

struct Game {
    a: Point,
    b: Point,
    prize: Point,
}

impl Game {
    fn some_if_div(top: i64, bottom: i64) -> Option<i64> {
        if top % bottom == 0 {
            Some(top / bottom)
        } else {
            None
        }
    }

    fn fn_b(&self) -> Option<i64> {
        let top = (self.a.y * self.prize.x) - (self.a.x * self.prize.y);
        let bottom = (self.a.y * self.b.x) - (self.a.x * self.b.y);
        Self::some_if_div(top, bottom)
    }

    fn fn_a(&self) -> Option<i64> {
        let top = (self.b.y * self.prize.x) - (self.b.x * self.prize.y);
        let bottom = (self.a.x * self.b.y) - (self.a.y * self.b.x);
        Self::some_if_div(top, bottom)
    }
}

pub struct GameParser;

impl Parser<std::vec::IntoIter<&str>, Game, anyhow::Error> for GameParser {
    fn parse_section(&self, mut other: std::vec::IntoIter<&str>) -> Result<Game> {
        Ok(Game {
            a: other
                .next()
                .context("Failed to get A line")?
                .parse()
                .context("failed to parse A line")?,
            b: other
                .next()
                .context("Failed to get A line")?
                .parse()
                .context("failed to parse A line")?,
            prize: other
                .next()
                .context("Failed to get Prize line")?
                .parse()
                .context("failed to parse Prize line")?,
        })
    }
}

fn run(input: &str, offset: i64) -> Result<Option<i64>> {
    let games: Vec<Game> = parse_input(LineGroupSplitter::blankline(), GameParser, input)
        .context("failed to parse input")?;

    let mut cost = 0;
    for mut game in games {
        game.prize.x += offset;
        game.prize.y += offset;
        if let (Some(a), Some(b)) = (game.fn_a(), game.fn_b()) {
            if a > 0 && b > 0 {
                let game_cost = a * 3 + b;
                cost += game_cost;
            }
        }
    }

    Ok(Some(cost))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<i64>, anyhow::Error> {
    run(input, 0)
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<i64>, anyhow::Error> {
    run(input, 10000000000000)
}

#[cfg(test)]
mod tests_day_13 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(480);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(875318608908);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
