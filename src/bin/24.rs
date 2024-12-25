#![allow(unused_imports)]
advent_of_code::solution!(24);

use advent_of_code::template::RunType;

use aoc_lib::grid::{CountingMap, Direction, Location, Map};
use aoc_lib::parse::preamble::*;

use anyhow::{anyhow, Context, Result};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use std::str::FromStr;

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
enum Mapping {
    Or { a: String, b: String },
    And { a: String, b: String },
    Xor { a: String, b: String },
}

impl Mapping {
    fn parse(other: &str) -> Result<(String, Self)> {
        let (ops, out) = other.split_once(" -> ").context("failed to split ->")?;
        let (a, rest) = ops.split_once(" ").context("failed to split out a")?;
        let (op, b) = rest.split_once(" ").context("failed to split out op")?;

        let a = a.to_string();
        let b = b.to_string();
        let out = out.to_string();

        let m = match op {
            "OR" => Mapping::Or { a, b },
            "AND" => Mapping::And { a, b },
            "XOR" => Mapping::Xor { a, b },
            _ => {
                return Err(anyhow!("got unexpected op: {:?}", op));
            }
        };

        Ok((out, m))
    }

    fn ab(&self) -> (&str, &str) {
        match self {
            Self::Or { a, b } => (a, b),
            Self::And { a, b } => (a, b),
            Self::Xor { a, b } => (a, b),
        }
    }

    fn matching_ab(&self, target: &str) -> Option<&str> {
        let (a, b) = self.ab();
        if a == target {
            return Some(b);
        } else if b == target {
            return Some(a);
        } else {
            return None;
        }
    }

    fn op_name(&self) -> &str {
        match self {
            Self::Or { .. } => "OR",
            Self::And { .. } => "AND",
            Self::Xor { .. } => "XOR",
        }
    }
}

fn resolve_wire(
    wire: &str,
    wires: &BTreeMap<String, Mapping>,
    inputs: &BTreeMap<String, bool>,
    cache: &mut BTreeMap<String, bool>,
    depth: usize,
    print: bool,
) -> Result<bool> {
    if let Some(input) = inputs.get(wire) {
        return Ok(*input);
    }
    if let Some(cached) = cache.get(wire) {
        return Ok(*cached);
    }

    let prefix: String = (0..depth).map(|_| ' ').collect();

    let map = wires.get(wire).context("expected to find wire!")?;
    if print {
        println!("{}{}={:?}", prefix, wire, map);
    }
    let value = match map {
        Mapping::And { a, b } => {
            let a_value = resolve_wire(&a, wires, inputs, cache, depth + 1, print)
                .context("failed to resolve AND a")?;
            let b_value = resolve_wire(&b, wires, inputs, cache, depth + 1, print)
                .context("failed to resolve AND b")?;

            a_value & b_value
        }
        Mapping::Or { a, b } => {
            let a_value = resolve_wire(&a, wires, inputs, cache, depth + 1, print)
                .context("failed to resolve OR a")?;
            let b_value = resolve_wire(&b, wires, inputs, cache, depth + 1, print)
                .context("failed to resolve OR b")?;

            a_value | b_value
        }
        Mapping::Xor { a, b } => {
            let a_value = resolve_wire(&a, wires, inputs, cache, depth + 1, print)
                .context("failed to resolve XOR a")?;
            let b_value = resolve_wire(&b, wires, inputs, cache, depth + 1, print)
                .context("failed to resolve XOR b")?;

            a_value ^ b_value
        }
    };

    cache.insert(wire.to_string(), value);
    Ok(value)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u64>, anyhow::Error> {
    let (inits, maps_str) = input
        .split_once("\n\n")
        .context("Failed to split sections")?;

    let mut inputs = BTreeMap::new();
    for line in inits.lines() {
        let (wire, value) = line
            .trim()
            .split_once(": ")
            .context("failed to split wire init line")?;

        let value = if value == "1" {
            true
        } else if value == "0" {
            false
        } else {
            return Err(anyhow!("Got unexpected wire value: {:?}", value));
        };

        inputs.insert(wire.to_string(), value);
    }

    let mut wires = BTreeMap::new();
    for map in maps_str.lines() {
        let (out, m) = Mapping::parse(map).context("failed to parse mapping")?;
        wires.insert(out, m);
    }

    let mut cache = BTreeMap::new();
    let mut out = 0;
    for wire in wires.keys() {
        if let Some(wire_num) = wire.strip_prefix('z') {
            if resolve_wire(wire, &wires, &inputs, &mut cache, 0, false)
                .context("failed to resolve wire")?
            {
                let v: u64 = wire_num.parse().context("failed to parse after z")?;
                out += 1 << v;
            }
        }
    }
    Ok(Some(out))
}

fn extract_xy(a: &str, b: &str) -> Option<(u64, u64)> {
    if let (Some(x), Some(y)) = (a.strip_prefix('x'), b.strip_prefix('y')) {
        Some((x.parse().unwrap(), y.parse().unwrap()))
    } else if let (Some(y), Some(x)) = (a.strip_prefix('y'), b.strip_prefix('x')) {
        Some((x.parse().unwrap(), y.parse().unwrap()))
    } else {
        None
    }
}

fn is_adder(idx: usize, wire: &str, wires: &BTreeMap<String, Mapping>) {
    let map = wires.get(wire).unwrap();
    println!("{} {:?}", idx, map);
    if let Mapping::Xor { a, b } = map {
        let a_map = wires.get(a).unwrap();
        let b_map = wires.get(b).unwrap();

        let sum = if let Mapping::Xor { .. } = a_map {
            a_map
        } else if let Mapping::Xor { .. } = b_map {
            b_map
        } else {
            println!("E: {} Can't find SUM XOR!", wire);
            return;
        };
        let cout = if let Mapping::Or { .. } = a_map {
            a_map
        } else if let Mapping::Or { .. } = b_map {
            b_map
        } else {
            println!("E: {} Can't find cout OR!", wire);
            return;
        };

        println!("{} has sum={:?} cout={:?}", wire, sum, cout);
        println!("{}={:?} {}={:?}", a, wires.get(a), b, wires.get(b));

        if let Mapping::Xor { a: sum_a, b: sum_b } = sum {
            if let Some((x, y)) = extract_xy(sum_a, sum_b) {
                assert!(x == y);
                if x != idx as u64 {
                    println!("E: {} Sum for {} uses {} as input", wire, idx, x);
                }
            } else {
                println!("Sum is not input wires!");
            }
        } else {
            unreachable!();
        }
    } else {
        println!("E: {} Top level must be an XOR!", wire);
    }
}

#[derive(Debug)]
enum GateTypeIdx {
    RawCarry(u64),
    RawSum(u64),
    InnerCarry(u64),
    OutCarry(u64),
    OutSum(u64),
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Hash)]
enum GateType {
    RawCarry,
    RawSum,
    InnerCarry,
    OutCarry,
    OutSum,
}

impl From<&Mapping> for (GateType, Option<u64>) {
    fn from(mapping: &Mapping) -> (GateType, Option<u64>) {
        match mapping {
            Mapping::Or { .. } => (GateType::OutCarry, None),
            Mapping::Xor { a, b } => {
                if let Some((x, y)) = extract_xy(a, b) {
                    assert!(x == y);
                    (GateType::RawSum, Some(x))
                } else {
                    (GateType::OutSum, None)
                }
            }
            Mapping::And { a, b } => {
                if let Some((x, y)) = extract_xy(a, b) {
                    assert!(x == y);
                    (GateType::RawCarry, Some(x))
                } else {
                    (GateType::InnerCarry, None)
                }
            }
        }
    }
}

#[derive(Debug)]
struct AddUnit {
    idx: u64,
    input_carry: String,
    raw_carry: String,
    raw_sum: String,
    inner_carry: String,
    out_sum: String,
    out_carry: String,
}

impl AddUnit {
    fn mappings(&self) -> Vec<(Mapping, String)> {
        let x = format!("x{:02}", self.idx);
        let y = format!("y{:02}", self.idx);
        let z = format!("z{:02}", self.idx);
        let mut links = Vec::new();
        links.push((
            Mapping::And {
                a: x.clone(),
                b: y.clone(),
            },
            self.raw_carry.clone(),
        ));
        links.push((
            Mapping::Xor {
                a: x.clone(),
                b: y.clone(),
            },
            self.raw_sum.clone(),
        ));
        links.push((
            Mapping::Xor {
                a: self.input_carry.clone(),
                b: self.raw_sum.clone(),
            },
            z,
        ));
        links.push((
            Mapping::Xor {
                a: self.input_carry.clone(),
                b: self.raw_sum.clone(),
            },
            self.out_sum.clone(),
        ));
        links.push((
            Mapping::And {
                a: self.input_carry.clone(),
                b: self.raw_sum.clone(),
            },
            self.inner_carry.clone(),
        ));
        links.push((
            Mapping::Or {
                a: self.inner_carry.clone(),
                b: self.raw_carry.clone(),
            },
            self.out_carry.clone(),
        ));

        links
    }
}

fn find(
    target: &str,
    t: GateType,
    by_type: &BTreeMap<GateType, BTreeSet<(String, Mapping)>>,
) -> Option<(String, String)> {
    for (wire, map) in by_type.get(&t).unwrap() {
        if let Some(other) = map.matching_ab(target) {
            return Some((wire.clone(), other.to_string()));
        }
    }

    None
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<String>, anyhow::Error> {
    let (inits, maps_str) = input
        .split_once("\n\n")
        .context("Failed to split sections")?;

    let mut inputs = BTreeMap::new();
    for line in inits.lines() {
        let (wire, value) = line
            .trim()
            .split_once(": ")
            .context("failed to split wire init line")?;

        let value = if value == "1" {
            true
        } else if value == "0" {
            false
        } else {
            return Err(anyhow!("Got unexpected wire value: {:?}", value));
        };

        inputs.insert(wire.to_string(), value);
    }

    let mut wires = BTreeMap::new();
    let mut wire_lookup = BTreeMap::new();
    let mut type_map = BTreeMap::new();
    let mut raw_carries = BTreeMap::new();
    let mut raw_sums = BTreeMap::new();
    let mut by_type: BTreeMap<GateType, BTreeSet<(String, Mapping)>> = BTreeMap::new();
    for map in maps_str.lines() {
        let (out, m) = Mapping::parse(map).context("failed to parse mapping")?;

        let (a, b) = m.ab();
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        wire_lookup.insert(
            (a.to_string(), m.op_name().to_string(), b.to_string()),
            out.clone(),
        );

        let (t, maybe_idx) = (&m).into();
        if let Some(idx) = maybe_idx {
            match t {
                GateType::RawCarry => {
                    raw_carries.insert(idx, out.clone());
                }
                GateType::RawSum => {
                    raw_sums.insert(idx, out.clone());
                }
                _ => {}
            }
        }

        by_type
            .entry(t.clone())
            .or_default()
            .insert((out.clone(), m.clone()));
        type_map.insert(out.clone(), t);
        wires.insert(out, m);
    }
    let mut cache = BTreeMap::new();
    resolve_wire(&"z13", &wires, &inputs, &mut cache, 0, false)
        .context("failed to resolve wire")?;
    println!("{:?}", cache);
    resolve_wire(&"z14", &wires, &inputs, &mut cache, 0, true).context("failed to resolve wire")?;
    println!("{:?}", cache);
    resolve_wire(&"z15", &wires, &inputs, &mut cache, 0, true).context("failed to resolve wire")?;
    println!("{:?}", cache);

    // let mut cache = BTreeMap::new();
    // resolve_wire(&"z02", &wires, &inputs, &mut cache, 0, true).context("failed to resolve wire")?;
    // println!("{:?}", cache);

    // let mut cache = BTreeMap::new();
    // resolve_wire(&"z03", &wires, &inputs, &mut cache, 0, true).context("failed to resolve wire")?;
    // println!("{:?}", cache);

    let mut units = Vec::new();
    let mut rewrite_carry = Vec::new();
    let mut prev_carry = "rtc".to_string();
    // let mut raw_half_adders = BTreeMap::new();
    for i in 2..45 {
        println!("prev_carry={}", prev_carry);
        let raw_carry = raw_carries.get(&i).unwrap();
        println!("raw_carry={}", raw_carry);
        let raw_sum = raw_sums.get(&i).unwrap();
        println!("raw_sum={}", raw_sum);
        // raw_half_adders.insert(i, (c, s));
        let inner_carry = match find(&prev_carry, GateType::InnerCarry, &by_type) {
            Some(c) => c.0,
            None => {
                panic!("failed!");
                let (wire, other) = find(&raw_sum, GateType::InnerCarry, &by_type).unwrap();
                rewrite_carry.push((i, prev_carry, other.clone()));
                prev_carry = other;
                wire
            }
        };
        // println!("inner_carry={}", inner_carry);
        let out_sum = find(&prev_carry, GateType::OutSum, &by_type).unwrap().0;
        // println!("out_sum={}", out_sum);
        let out_carry = match find(&inner_carry, GateType::OutCarry, &by_type) {
            Some(o) => o.0,
            None => {
                let mut cache = BTreeMap::new();
                resolve_wire(&prev_carry, &wires, &inputs, &mut cache, 0, false)
                    .context("failed to resolve wire")?;
                resolve_wire(&format!("z{:02}", i), &wires, &inputs, &mut cache, 0, true)
                    .context("failed to resolve wire")?;
                println!("{:?}", cache);
                panic!("failed!");
                // let (wire, other) = find(&raw_carry, GateType::OutCarry, &by_type).unwrap();
                // rewrite_carry.push((i, prev_carry, other.clone()));
                // prev_carry = other;
                // wire
            }
        };
        let unit = AddUnit {
            idx: i as u64,
            input_carry: prev_carry.clone(),
            raw_carry: raw_carry.clone(),
            raw_sum: raw_sum.clone(),
            inner_carry,
            out_sum,
            out_carry: out_carry.clone(),
        };
        println!("{}={:?}", i, unit);
        units.push(unit);
        prev_carry = out_carry;
    }

    let mut broken = BTreeSet::new();
    for unit in units {
        for (mapping, out) in unit.mappings() {
            let (a, b) = mapping.ab();
            let (a, b) = if a < b { (a, b) } else { (b, a) };
            let op = mapping.op_name();
            if let Some(input_wire) =
                wire_lookup.get(&(a.to_string(), op.to_string(), b.to_string()))
            {
                if input_wire != &out {
                    println!("{} {} {} -> {} != {}", a, op, b, out, input_wire);
                    broken.insert(input_wire.clone());
                    broken.insert(out);
                } else {
                    // println!("{} {} {} -> {}", a, op, b, out);
                }
            } else {
                println!("{}: {} {} {} -> {}", unit.idx, a, op, b, out);
                println!("Failed to find link!");
            }
        }
        if !unit.out_sum.starts_with("z") {
            broken.insert(unit.out_sum.clone());
            broken.insert(format!("z{:02}", unit.idx));
        }
        if unit.raw_carry.starts_with("z") {
            broken.insert(unit.raw_carry.clone());
        }
        if unit.raw_sum.starts_with("z") {
            broken.insert(unit.raw_sum.clone());
        }
        if unit.inner_carry.starts_with("z") {
            broken.insert(unit.inner_carry.clone());
        }
        if unit.out_carry.starts_with("z") {
            broken.insert(unit.out_carry.clone());
        }
    }

    for (_, a, b) in &rewrite_carry {
        broken.insert(a.clone());
        broken.insert(b.clone());
    }
    println!("{:?}", rewrite_carry);
    println!("{:?} {}", broken, broken.len());

    let mut out: Vec<String> = broken.into_iter().collect();
    out.sort();

    Ok(Some(out.join(",")))

    // let mut type_map = BTreeMap::new();
    // let mut raw_carry = BTreeMap::new();
    // let mut raw_sums = BTreeMap::new();
    // let mut inner_carry = BTreeMap::new();
    // let mut out_carry = BTreeMap::new();
    // let mut out_sum = BTreeMap::new();

    // for (wire, mapping) in &wires {
    //     if let Mapping::And { a, b } = mapping {
    //         if let Some((x, y)) = extract_xy(a, b) {
    //             assert!(x == y);
    //             raw_carry.insert(x, wire.clone());
    //             assert!(!type_map.contains_key(wire));
    //             type_map.insert(wire.clone(), GateTypeIdx::RawCarry(x));
    //         }
    //     }
    //     if let Mapping::Xor { a, b } = mapping {
    //         if let Some((x, y)) = extract_xy(a, b) {
    //             assert!(x == y);
    //             raw_sums.insert(x, wire.clone());
    //             assert!(!type_map.contains_key(wire));
    //             type_map.insert(wire.clone(), GateTypeIdx::RawSum(x));
    //         }
    //     }
    // }
    // for (wire, mapping) in &wires {
    //     if let Mapping::And { a, b } = mapping {
    //         // If we find an AND and it's not a raw carry it must be an InnerCarry
    //         if type_map.contains_key(wire) {
    //             continue;
    //         }

    //         let (_, idx) = if let Some(GateTypeIdx::RawSum(idx)) = type_map.get(a) {
    //             (a, idx)
    //         } else if let Some(GateTypeIdx::RawSum(idx)) = type_map.get(b) {
    //             (b, idx)
    //         } else {
    //             println!(
    //                 "{}={:?} should be a valid InnerCarry but isn't",
    //                 wire, mapping
    //             );
    //             continue;
    //         };
    //         inner_carry.insert(*idx, wire.clone());
    //         assert!(!type_map.contains_key(wire));
    //         type_map.insert(wire.clone(), GateTypeIdx::InnerCarry(*idx));
    //     }
    // }
    // println!("raw_carry={:?}", raw_carry);
    // println!("raw_sums={:?}", raw_sums);
    // println!("inner_carry={:?}", inner_carry);
    // println!("{:?}", type_map);

    // for (wire, mapping) in &wires {
    //     // All ORs should be for outcarry
    //     if let Mapping::Or { a, b } = mapping {
    //         assert!(!type_map.contains_key(wire));

    //         let a_type = if let Some(a_type) = type_map.get(a) {
    //             a_type
    //         } else {
    //             println!("{} a ({}) is unknown!", wire, a);
    //             continue;
    //         };
    //         let b_type = if let Some(b_type) = type_map.get(b) {
    //             b_type
    //         } else {
    //             println!("{} b ({}) is unknown!", wire, b);
    //             continue;
    //         };
    //         let idx = match (a_type, b_type) {
    //             (GateTypeIdx::InnerCarry(inner_idx), GateTypeIdx::RawCarry(raw_idx))
    //             | (GateTypeIdx::RawCarry(raw_idx), GateTypeIdx::InnerCarry(inner_idx)) => {
    //                 assert!(raw_idx == inner_idx);
    //                 raw_idx
    //             }
    //             _ => {
    //                 println!(
    //                     "{} got unexpected types {:?} and {:?}",
    //                     wire, a_type, b_type
    //                 );
    //                 continue;
    //             }
    //         };

    //         out_carry.insert(*idx, wire.clone());
    //         assert!(!type_map.contains_key(wire));
    //         type_map.insert(wire.clone(), GateTypeIdx::OutCarry(*idx));
    //     }
    // }

    // for (wire, mapping) in &wires {
    //     // All ORs should be for outcarry
    //     if let Mapping::Xor { a, b } = mapping {
    //         if type_map.contains_key(wire) {
    //             continue;
    //         }

    //         let a_type = if let Some(a_type) = type_map.get(a) {
    //             a_type
    //         } else {
    //             println!("{} a ({}) is unknown!", wire, a);
    //             continue;
    //         };
    //         let b_type = if let Some(b_type) = type_map.get(b) {
    //             b_type
    //         } else {
    //             println!("{} b ({}) is unknown!", wire, b);
    //             continue;
    //         };
    //         let idx = match (a_type, b_type) {
    //             (GateTypeIdx::OutCarry(carry_idx), GateTypeIdx::RawSum(sum_idx))
    //             | (GateTypeIdx::RawSum(sum_idx), GateTypeIdx::OutCarry(carry_idx)) => {
    //                 assert!(carry_idx + 1 == *sum_idx);
    //                 sum_idx
    //             }
    //             _ => {
    //                 println!(
    //                     "{} got unexpected types {:?} and {:?}",
    //                     wire, a_type, b_type
    //                 );
    //                 continue;
    //             }
    //         };

    //         out_sum.insert(*idx, wire.clone());
    //         assert!(!type_map.contains_key(wire));
    //         type_map.insert(wire.clone(), GateTypeIdx::OutSum(*idx));

    //         println!(
    //             "{}={:?}: {:?} {:?}",
    //             wire,
    //             mapping,
    //             type_map.get(a),
    //             type_map.get(b)
    //         );
    //     }
    // }

    // for (wire, mapping) in &wires {
    //     if type_map.contains_key(wire) {
    //         continue;
    //     }
    //     println!("Unmapped: {}={:?}", wire, mapping);
    // }
    // for i in 2..45 {
    //     let wire = format!("z{:02}", i);
    //     let gate_type = type_map.get(&wire);
    //     if let Some(GateTypeIdx::OutSum(idx)) = gate_type {
    //         if *idx == i {
    //             continue;
    //         }
    //         println!(
    //             "{} ({:?}) end is correct type but bad index! {}",
    //             wire, gate_type, idx
    //         );
    //     } else {
    //         println!("{} ({:?}) end is invalid!", wire, gate_type);
    //     }
    // }
    // for i in 2..45 {
    //     let elements = [
    //         raw_carry.get(&i),
    //         raw_sums.get(&i),
    //         inner_carry.get(&i),
    //         out_carry.get(&i),
    //         out_sum.get(&i),
    //     ];
    //     let mut invalid = elements.iter().any(|e| e.is_none());
    //     if let Some(s) = out_sum.get(&i) {
    //         invalid |= !s.starts_with('z');
    //     }
    //     if invalid {
    //         println!(
    //             "S: {} {:?} {:?} {:?} {:?} {:?}",
    //             i,
    //             raw_carry.get(&i),
    //             raw_sums.get(&i),
    //             inner_carry.get(&i),
    //             out_carry.get(&i),
    //             out_sum.get(&i),
    //         );
    //     }
    //     let wire = format!("z{:02}", i);
    //     if let Some(GateTypeIdx::OutSum(idx)) = type_map.get(&wire) {
    //         if *idx != i {
    //             println!("end is correct type but bad index! {}", idx);
    //         }
    //     } else {
    //         println!("end is invalid!");
    //     }
    // }

    // let mut out_carry = BTreeMap::new();

    // for wire in wires.keys() {
    // for i in 2..45 {
    //     let mut cache = BTreeMap::new();
    //     let wire = format!("z{:02}", i);
    //     println!("{:?}", wire);
    //     resolve_wire(&wire, &wires, &mut cache, 0, false).context("failed to resolve wire")?;
    //     let mut inputs = Vec::new();
    //     for key in cache.keys() {
    //         if key.starts_with("x") || key.starts_with("y") || key.starts_with("z") {
    //             inputs.push(key);
    //         }
    //     }
    //     // println!("{}, {:?}", wire, inputs);
    //     is_adder(i, &wire, &wires);
    // }

    // let mut cache = BTreeMap::new();
    // resolve_wire(&"z06", &wires, &inputs, &mut cache, 0, true).context("failed to resolve wire")?;
    // println!("{:?}", cache);
    // let mut cache = BTreeMap::new();
    // for wire in wires.keys() {
    //     if let Some(_) = wire.strip_prefix('z') {
    //         resolve_wire(wire, &wires, &mut cache, 0, false).context("failed to resolve wire")?;
    //     }
    // }

    // is_adder(1, "z01", &wires);
    // println!("{} vs {}", cache.len(), wires.len());

    // let mut out = 0;
    // for wire in wires.keys() {
    //     if let Some(wire_num) = wire.strip_prefix('z') {
    //         if resolve_wire(wire, &wires, &mut cache).context("failed to resolve wire")? {
    //             let v: u64 = wire_num.parse().context("failed to parse after z")?;
    //             out += 1 << v;
    //         }
    //     }
    // }
    //

    // hdt <-> z05
    // Ok(Some(out))
}

#[cfg(test)]
mod tests_day_24 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(2024);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = None;
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
