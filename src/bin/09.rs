advent_of_code::solution!(9);

use advent_of_code::template::RunType;

use anyhow::{Context, Result};

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let data: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let size: u32 = data.iter().sum();
    let mut total = Vec::with_capacity(size as usize);

    let mut data_blocks = 0;
    for (id, chunk) in data.chunks(2).enumerate() {
        for _ in 0..chunk[0] {
            data_blocks += 1;
            total.push(Some(id as u64));
        }
        if chunk.len() > 1 {
            for _ in 0..chunk[1] {
                total.push(None);
            }
        }
    }

    let mut rev = total.iter().rev().filter_map(|v| v.as_ref());
    let mut checksum: u64 = 0;
    for (idx, item) in total.iter().enumerate().take(data_blocks) {
        let v = match item {
            Some(v) => *v,
            None => *rev.next().context("expected next value")?,
        };
        checksum += idx as u64 * v;
    }

    Ok(Some(checksum))
}

#[derive(Debug)]
struct Block {
    start: usize,
    len: usize,
}

#[derive(Debug)]
struct File {
    id: usize,
    block: Block,
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let data: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut idx = 0;
    let mut file_list: Vec<File> = Vec::new();
    let mut free_list: Vec<Block> = Vec::new();
    for (id, chunk) in data.chunks(2).enumerate() {
        file_list.push(File {
            id,
            block: Block {
                start: idx,
                len: chunk[0] as usize,
            },
        });
        idx += chunk[0] as usize;
        if chunk.len() > 1 {
            free_list.push(Block {
                start: idx,
                len: chunk[1] as usize,
            });
            idx += chunk[1] as usize;
        }
    }

    let mut checksum: u64 = 0;
    for mut file in file_list.into_iter().rev() {
        for free in free_list.iter_mut() {
            if free.start >= file.block.start {
                break;
            }
            if free.len >= file.block.len {
                free.len -= file.block.len;
                file.block.start = free.start;
                free.start += file.block.len;
                break;
            }
        }
        for i in 0..file.block.len {
            checksum = checksum
                .checked_add((file.block.start + i) as u64 * file.id as u64)
                .expect("Overflow!");
        }
    }

    Ok(Some(checksum))
}

#[cfg(test)]
mod tests_day_9 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(1928);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(2858);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
