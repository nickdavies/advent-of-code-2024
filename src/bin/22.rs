advent_of_code::solution!(22);

use advent_of_code::template::RunType;

use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

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

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let numbers: Vec<u64> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
        .context("failed to parse input")?;

    let mut data = Vec::new();
    for number in numbers {
        let mut price_lookup: BTreeMap<Vec<i64>, u64> = BTreeMap::new();
        let mut prices: Vec<(u64, Vec<i64>)> = Vec::new();
        let mut deltas = VecDeque::new();
        let secrets = SecretGen::new(number).take(2000);
        let mut prev: i64 = number as i64 % 10;
        for secret in secrets {
            let price = secret % 10;
            deltas.push_back(prev - price as i64);
            prev = price as i64;
            match deltas.len() {
                0..=4 => continue,
                5 => {
                    deltas.pop_front();
                    let delta_vec: Vec<i64> = deltas.iter().cloned().collect();
                    if price_lookup.contains_key(&delta_vec) {
                        continue;
                    }
                    price_lookup.insert(delta_vec.clone(), price);
                    prices.push((price, delta_vec));
                }
                _ => unreachable!(),
            }
        }
        prices.sort_by_key(|v| std::cmp::Reverse(v.0));
        data.push((prices, price_lookup));
    }

    let mut seen: BTreeSet<Vec<i64>> = BTreeSet::new();
    let mut best: Option<u64> = None;
    let mut best_seq: Option<Vec<i64>> = None;
    for (current, (prices, _)) in data.iter().enumerate() {
        println!("Checking: {}", current);
        for (current_price, delta) in prices {
            if seen.contains(delta) {
                continue;
            }
            let mut total = *current_price;
            for (other, (_, price_lookup)) in data.iter().enumerate() {
                if current == other {
                    continue;
                }
                if let Some(other_price) = price_lookup.get(delta) {
                    total += other_price;
                }
            }
            if best.unwrap_or(total) <= total {
                best = Some(total);
                best_seq = Some(delta.clone());
            }
            seen.insert(delta.clone());
        }
    }

    let best = best.unwrap();
    let best_seq = best_seq.unwrap();
    println!("{} {:?}", best, best_seq);
    Ok(Some(best))
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
