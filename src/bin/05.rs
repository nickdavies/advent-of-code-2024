advent_of_code::solution!(5);

use advent_of_code::template::RunType;

use anyhow::{Context, Result};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct PageOrder {
    forward: BTreeMap<i32, BTreeSet<i32>>,
    backward: BTreeMap<i32, BTreeSet<i32>>,
}

impl PageOrder {
    fn new(p1: Vec<(i32, i32)>) -> Self {
        let mut forward = BTreeMap::new();
        let mut backward = BTreeMap::new();

        for (left, right) in p1 {
            forward
                .entry(left)
                .or_insert_with(BTreeSet::new)
                .insert(right);
            backward
                .entry(right)
                .or_insert_with(BTreeSet::new)
                .insert(left);
        }

        Self { forward, backward }
    }

    fn cmp(&self, a: &i32, b: &i32) -> Ordering {
        if a == b {
            return Ordering::Equal;
        }
        match self.forward.get(a) {
            Some(afters) => {
                if afters.contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            None => match self.backward.get(a) {
                Some(befores) => {
                    if befores.contains(b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
                None => unreachable!(),
            },
        }
    }
}

pub struct Update<'a> {
    data: Vec<i32>,
    ord: &'a PageOrder,
}

impl<'a> Update<'a> {
    fn new(data: Vec<i32>, ord: &'a PageOrder) -> Self {
        Self { data, ord }
    }

    fn valid(&self) -> bool {
        for win in self.data.windows(2) {
            if let Ordering::Greater = self.ord.cmp(&win[0], &win[1]) {
                return false;
            }
        }
        true
    }

    fn mid(&self) -> i32 {
        let mid = self.data.len() / 2;
        self.data[mid]
    }

    fn fix(&mut self) {
        self.data.sort_by(|a, b| self.ord.cmp(a, b));
    }
}

pub fn parse_section_1(input: &str) -> Result<(PageOrder, std::str::Lines)> {
    let mut lines = input.lines();
    let mut out = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        let (a, b) = line.split_once("|").context("failed to split line")?;
        out.push((
            a.parse().context("Failed to parse first number")?,
            b.parse().context("failed to parse second number")?,
        ));
    }

    Ok((PageOrder::new(out), lines))
}

pub fn parse_section_2<'a>(lines: std::str::Lines, ord: &'a PageOrder) -> Result<Vec<Update<'a>>> {
    let mut out = Vec::new();
    for line in lines {
        let mut entries = Vec::new();
        for num in line.split(",") {
            entries.push(num.parse().context("Failed to parse one of the numbers")?);
        }
        out.push(Update::new(entries, ord));
    }
    Ok(out)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<i32>, anyhow::Error> {
    let (ord, lines) = parse_section_1(input).context("failed to parse part 1")?;
    let updates = parse_section_2(lines, &ord).context("failed to parse part 2")?;

    Ok(Some(
        updates
            .iter()
            .filter_map(|u| if u.valid() { Some(u.mid()) } else { None })
            .sum(),
    ))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<i32>, anyhow::Error> {
    let (ord, lines) = parse_section_1(input).context("failed to parse part 1")?;
    let mut updates = parse_section_2(lines, &ord).context("failed to parse part 2")?;

    Ok(Some(
        updates
            .iter_mut()
            .filter_map(|u| {
                if u.valid() {
                    None
                } else {
                    u.fix();
                    Some(u.mid())
                }
            })
            .sum(),
    ))
}

#[cfg(test)]
mod tests_day_5 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, Some(143));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, Some(123));
        Ok(())
    }
}
