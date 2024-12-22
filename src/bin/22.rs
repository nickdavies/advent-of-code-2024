advent_of_code::solution!(22);

use advent_of_code::template::RunType;

use anyhow::{Context, Result};
use std::collections::VecDeque;

struct SecretGen {
    secret: u64,
}

impl SecretGen {
    fn new(secret: u64) -> Self {
        Self { secret }
    }
}

impl Iterator for SecretGen {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let step1 = (self.secret ^ (self.secret * 64)) % 16777216;
        let step2 = (step1 ^ (step1 / 32)) % 16777216;
        let step3 = (step2 ^ (step2 * 2048)) % 16777216;
        self.secret = step3;
        Some(step3)
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let numbers: Vec<u64> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
        .context("failed to parse input")?;

    let mut out = 0;
    for number in numbers {
        let initial = number;
        let secret = SecretGen::new(initial).nth(1999).unwrap();
        out += secret;
    }
    Ok(Some(out))
}

const MAX_NUMBERS: usize = 2000;

#[derive(Debug)]
struct ComboCache {
    total: u64,
    prices: [Option<u64>; MAX_NUMBERS],
}

impl ComboCache {
    fn new() -> Self {
        Self {
            total: 0,
            prices: [None; MAX_NUMBERS],
        }
    }

    fn add(&mut self, number: usize, price: u64) -> u64 {
        if self.prices[number].is_none() {
            self.prices[number] = Some(price);
            self.total += price;
        }
        self.total
    }
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let numbers: Vec<u64> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
        .context("failed to parse input")?;

    let mut cache_table: Vec<Vec<Vec<Vec<ComboCache>>>> = Vec::with_capacity(20);
    for _ in 0..20 {
        let mut l1 = Vec::with_capacity(20);
        for _ in 0..20 {
            let mut l2 = Vec::with_capacity(20);
            for _ in 0..20 {
                let mut l3 = Vec::with_capacity(20);
                for _ in 0..20 {
                    l3.push(ComboCache::new());
                }
                l2.push(l3);
            }
            l1.push(l2);
        }
        cache_table.push(l1);
    }
    // You can replace this monstrosity with below. It's slowe but doesn't depend on
    // the exact AoC problem details as much.
    // let mut cache: BTreeMap<Vec<u64>, ComboCache> = BTreeMap::new();
    let mut best: Option<u64> = None;
    for (number_id, number) in numbers.iter().enumerate() {
        let mut deltas = VecDeque::new();
        let secrets = SecretGen::new(*number).take(2000);
        let mut prev: i64 = *number as i64 % 10;
        for secret in secrets {
            let price = secret % 10;
            deltas.push_back(prev - price as i64);
            prev = price as i64;
            match deltas.len() {
                0..=4 => continue,
                5 => {
                    deltas.pop_front();
                    let mut delta_iter = deltas.iter();
                    let cache_entry = &mut cache_table[(*delta_iter.next().unwrap() + 10) as usize]
                        [(*delta_iter.next().unwrap() + 10) as usize]
                        [(*delta_iter.next().unwrap() + 10) as usize]
                        [(*delta_iter.next().unwrap() + 10) as usize];

                    let combo_price = cache_entry.add(number_id, price);
                    best = Some(std::cmp::max(best.unwrap_or(combo_price), combo_price));
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(Some(best.unwrap()))
}

#[cfg(test)]
mod tests_day_22 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(37327623);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(23);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");

        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
