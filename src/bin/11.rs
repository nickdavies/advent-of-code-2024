advent_of_code::solution!(11);

use advent_of_code::template::RunType;

use aoc_lib::parse::preamble::*;

use anyhow::Result;
use std::collections::BTreeMap;

fn blink_once(data: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(data.len());
    for stone in data {
        if *stone == 0 {
            out.push(1);
            continue;
        }

        let stone_s = format!("{}", stone);
        if stone_s.len() % 2 == 0 {
            let (a, b) = SplitMiddle(Identity, Identity)
                .parse_section(&stone_s)
                .unwrap();
            out.push(a.parse().unwrap());
            out.push(b.parse().unwrap());
        } else {
            out.push(stone * 2024);
        }
    }
    out
}

fn blink_n(num: u64, n: usize) -> Vec<u64> {
    let mut data = vec![num];
    for _ in 0..n {
        data = blink_once(&data);
    }
    data
}

fn blink_from_cache(
    num: u64,
    n: usize,
    n_per: usize,
    cache: &mut BTreeMap<(u64, usize), u64>,
) -> u64 {
    if let Some(total) = cache.get(&(num, n)) {
        return *total;
    }
    if n == 0 {
        return 1;
    }

    let mut total = 0;
    for value in blink_n(num, n_per) {
        total += blink_from_cache(value, n - 1, n_per, cache);
    }

    cache.insert((num, n), total);
    total
}

pub fn run(input: &str, n: usize) -> Result<u64> {
    let data: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cache = BTreeMap::new();
    let mut total = 0;
    for value in data {
        total += blink_from_cache(value, n / 5, 5, &mut cache);
    }

    Ok(total)
}
pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    Ok(Some(run(input, 25)?))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    Ok(Some(run(input, 75)?))
}

#[cfg(test)]
mod tests_day_11 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(55312);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(65601038650482);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
