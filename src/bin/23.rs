advent_of_code::solution!(23);

use advent_of_code::template::RunType;

use aoc_lib::parse::preamble::*;

use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};

fn build_connections(input: &str) -> Result<BTreeMap<&str, BTreeSet<&str>>> {
    let data: Vec<(&str, &str)> =
        parse_input(LineSplitter, ParseTuple2(Identity, Identity, "-"), input)
            .context("failed to parse input")?;

    let mut connections: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (source, dest) in data {
        connections.entry(source).or_default().insert(dest);
        connections.entry(dest).or_default().insert(source);
    }
    Ok(connections)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let connections = build_connections(input)?;

    let mut out: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    for a in connections.keys() {
        if !a.starts_with("t") {
            continue;
        }
        if let Some(a_targets) = connections.get(a) {
            for b in a_targets {
                if a == b {
                    continue;
                }
                if let Some(b_targets) = connections.get(b) {
                    for c in b_targets.intersection(a_targets) {
                        if !a_targets.contains(c) {
                            continue;
                        }
                        if c == b || c == a {
                            continue;
                        }
                        out.insert(
                            [a.to_string(), b.to_string(), c.to_string()]
                                .into_iter()
                                .collect(),
                        );
                    }
                }
            }
        }
    }

    Ok(Some(out.len()))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<String>, anyhow::Error> {
    let connections = build_connections(input)?;

    let mut hist: BTreeMap<String, usize> = BTreeMap::new();
    for (a, a_dests) in &connections {
        for (b, b_dests) in &connections {
            if a == b {
                continue;
            }
            let mut inter: Vec<&str> = a_dests.intersection(b_dests).cloned().collect();
            if !(a_dests.contains(a) && b_dests.contains(a)) {
                inter.push(a);
            }
            if !(a_dests.contains(b) && b_dests.contains(b)) {
                inter.push(b);
            }
            inter.sort();
            *hist.entry(inter.join(",")).or_default() += 1;
        }
    }

    let (max_group, _) = hist.into_iter().max_by_key(|(_, v)| *v).unwrap();
    Ok(Some(max_group))
}

#[cfg(test)]
mod tests_day_23 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(7);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some("co,de,ka,ta".to_string());
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
