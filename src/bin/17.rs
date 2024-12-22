advent_of_code::solution!(17);

use advent_of_code::template::RunType;

use anyhow::{anyhow, Context, Result};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Registers([u64; 3]);

impl Registers {
    fn a(&self) -> u64 {
        self.0[0]
    }
    fn a_mut(&mut self) -> &mut u64 {
        &mut self.0[0]
    }

    fn b(&self) -> u64 {
        self.0[1]
    }
    fn b_mut(&mut self) -> &mut u64 {
        &mut self.0[1]
    }

    fn c(&self) -> u64 {
        self.0[2]
    }

    fn c_mut(&mut self) -> &mut u64 {
        &mut self.0[2]
    }
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Literal(u8);

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct Combo(u8);

impl Combo {
    fn new(value: u8) -> Result<Self> {
        if value > 7 {
            return Err(anyhow!("Invalid combo value: {}", value));
        }

        Ok(Self(value))
    }

    fn resolve(&self, registers: &Registers) -> u64 {
        match self.0 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => registers.a(),
            5 => registers.b(),
            6 => registers.c(),
            _ => unreachable!(),
        }
    }

    fn value_str(&self) -> &str {
        match self.0 {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "%A",
            5 => "%B",
            6 => "%C",
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub enum Op {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc,
    Out(Combo),
    Bdb(Combo),
    Cdv(Combo),
}

impl Op {
    fn name(&self) -> &str {
        match self {
            Self::Adv(_) => "Adv",
            Self::Bxl(_) => "Bxl",
            Self::Bst(_) => "Bst",
            Self::Jnz(_) => "Jnz",
            Self::Bxc => "Bxc",
            Self::Out(_) => "Out",
            Self::Bdb(_) => "Bdb",
            Self::Cdv(_) => "Cdv",
        }
    }

    fn operand_str(&self) -> String {
        match self {
            Self::Adv(combo)
            | Self::Bst(combo)
            | Self::Out(combo)
            | Self::Bdb(combo)
            | Self::Cdv(combo) => combo.value_str().to_string(),
            Self::Bxl(literal) | Self::Jnz(literal) => format!("{}", literal.0),
            Self::Bxc => String::new(),
        }
    }

    fn run(&self, registers: &mut Registers, ip: u64) -> (Option<u8>, u64) {
        match self {
            Self::Adv(combo) => {
                *registers.a_mut() = registers.a() / (2_u64).pow(combo.resolve(registers) as u32);
            }
            Self::Bxl(literal) => {
                *registers.b_mut() = registers.b() ^ literal.0 as u64;
            }
            Self::Bst(combo) => {
                *registers.b_mut() = combo.resolve(registers) % 8;
            }
            Self::Jnz(literal) => {
                if registers.a() != 0 {
                    return (None, literal.0 as u64 / 2);
                }
            }
            Self::Bxc => {
                *registers.b_mut() = registers.b() ^ registers.c();
            }
            Self::Out(combo) => {
                return (Some((combo.resolve(registers) % 8) as u8), ip + 1);
            }
            Self::Bdb(combo) => {
                *registers.b_mut() = registers.a() / (2_u64).pow(combo.resolve(registers) as u32);
            }
            Self::Cdv(combo) => {
                *registers.c_mut() = registers.a() / (2_u64).pow(combo.resolve(registers) as u32);
            }
        }

        (None, ip + 1)
    }
}

impl TryFrom<(u8, u8)> for Op {
    type Error = anyhow::Error;

    fn try_from(other: (u8, u8)) -> Result<Self> {
        let (op_id, operand) = other;

        Ok(match op_id {
            0 => Op::Adv(Combo::new(operand)?),
            1 => Op::Bxl(Literal(operand)),
            2 => Op::Bst(Combo::new(operand)?),
            3 => Op::Jnz(Literal(operand)),
            4 => Op::Bxc,
            5 => Op::Out(Combo::new(operand)?),
            6 => Op::Bdb(Combo::new(operand)?),
            7 => Op::Cdv(Combo::new(operand)?),
            other => {
                return Err(anyhow!("found unexpected Op value {:?}", other));
            }
        })
    }
}

fn parse(input: &str) -> Result<(Vec<u8>, BTreeMap<char, u64>)> {
    let (reg_str, program_str) = input
        .split_once("\n\n")
        .context("Expected to find 2 sections")?;

    let ops: Vec<u8> = program_str
        .trim()
        .strip_prefix("Program: ")
        .context("Expected program to start with Program:")?
        .split(",")
        .map(|v| {
            v.parse()
                .context(format!("failed to parse program int {:?}", v))
        })
        .collect::<Result<Vec<_>>>()?;

    let regs: BTreeMap<char, u64> = reg_str
        .lines()
        .map(|l| {
            let (reg, value) = l
                .strip_prefix("Register ")
                .context("expected register prefix")?
                .split_once(": ")
                .context("failed to split register init line")?;
            Ok((
                reg.chars()
                    .next()
                    .context("expected to find char from reg")?,
                value
                    .parse::<u64>()
                    .context("failed to parse register initial value")?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;

    Ok((ops, regs))
}

fn run_program(program: &[Op], mut registers: Registers) -> Vec<u8> {
    let mut out = Vec::new();
    let mut ip: u64 = 0;
    while (ip as usize) < program.len() {
        let op = &program[ip as usize];
        let ret = op.run(&mut registers, ip);
        if let Some(cmd_out) = ret.0 {
            out.push(cmd_out);
        }
        ip = ret.1;
    }
    out
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<String>, anyhow::Error> {
    let (program_ops, registers_map) = parse(input)?;

    let mut program = Vec::new();
    for chunk in program_ops.chunks(2) {
        program.push(Op::try_from((chunk[0], chunk[1])).context("failed to build op")?);
    }

    let registers = Registers([
        *registers_map
            .get(&'A')
            .context("expected to find the A register")?,
        *registers_map
            .get(&'B')
            .context("expected to find the B register")?,
        *registers_map
            .get(&'C')
            .context("expected to find the C register")?,
    ]);

    let out = run_program(&program, registers);
    let out_str: Vec<String> = out.into_iter().map(|v| format!("{}", v)).collect();
    Ok(Some(out_str.join(",")))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let (program_ops, registers_map) = parse(input)?;

    let mut program = Vec::new();
    for chunk in program_ops.chunks(2) {
        program.push(Op::try_from((chunk[0], chunk[1])).context("failed to build op")?);
    }
    let input_registers = Registers([
        0,
        *registers_map
            .get(&'B')
            .context("expected to find the B register")?,
        *registers_map
            .get(&'C')
            .context("expected to find the C register")?,
    ]);

    let mut cycle: u64 = 0;
    loop {
        let mut regs = input_registers.clone();
        *regs.a_mut() = cycle;

        let output = run_program(&program, regs);
        if output.len() > 1 {
            break;
        }
        cycle += 1;
    }

    let mut target_digit = program_ops.len() - 1;
    let mut possible_a = cycle.pow(target_digit as u32);
    loop {
        let mut regs = input_registers.clone();
        *regs.a_mut() = possible_a;

        let output = run_program(&program, regs);
        println!("{:?} = {}", output, possible_a);

        if output == program_ops {
            break;
        }

        if output[target_digit] == program_ops[target_digit] {
            println!(
                "HERE: {} stepping by {}",
                target_digit,
                cycle.pow(target_digit as u32)
            );
            if target_digit == 0 {
                break;
            }
            target_digit -= 1;
        } else {
            possible_a += cycle.pow(target_digit as u32);
        }
    }

    for i in 0..100000 {
        let mut regs = input_registers.clone();
        *regs.a_mut() = possible_a + i;
        let output = run_program(&program, regs);
        println!("{:?} = {}", output, possible_a + i);

        if output == program_ops {
            println!("I={}", i);
            break;
        }
    }

    for op in program {
        println!("{} {}", op.name(), op.operand_str());
    }

    // println!("HERE: {}", i);

    Ok(None)
}

#[cfg(test)]
mod tests_day_17 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some("4,6,3,5,6,3,5,2,1,0".to_string());
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(117440);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
