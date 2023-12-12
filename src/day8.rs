use std::collections::{HashMap, HashSet};

use crate::Error;

type DesertMap = HashMap<String, (String, String)>;

fn parse_map(input: Vec<String>) -> anyhow::Result<(String, DesertMap)> {
    let directions = input[0].clone();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..input.len() {
        let (node, left_right_str) = input[i].split_once('=').ok_or(Error::InvalidInput(
            format!("Invalid format on line: {}", input[i]),
        ))?;

        let replaced = left_right_str.replace('(', "").replace(')', "");

        let (left, right) =
            replaced
                .trim()
                .split_once(", ")
                .ok_or(Error::InvalidInput(format!(
                    "Invalid format on line: {}",
                    input[i]
                )))?;

        map.insert(
            node.trim().to_string(),
            (left.to_string(), right.to_string()),
        );
    }

    Ok((directions, map))
}

fn take_step(
    node: &String,
    directions: &String,
    direction_index: usize,
    map: &DesertMap,
) -> anyhow::Result<String> {
    let (left, right) = map
        .get(node)
        .ok_or(Error::InvalidInput(format!("{} is not in map", node)))?;
    match directions
        .chars()
        .nth(direction_index)
        .ok_or(Error::InvalidInput("How did i get here".into()))?
    {
        'R' => Ok(right.clone()),
        'L' => Ok(left.clone()),
        other => {
            return Err(Error::InvalidInput(format!("{} Should not be in direction", other)).into())
        }
    }
}

fn find_sleep(start: String, directions: &String, map: &DesertMap) -> anyhow::Result<u64> {
    let mut current = start;
    let mut direction_index = 0;
    let mut steps = 0;

    while !current.ends_with('Z') {
        current = take_step(&current, directions, direction_index, map)?;
        steps += 1;
        direction_index = (direction_index + 1) % directions.len();
    }

    Ok(steps)
}

fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    for i in (5..).step_by(6).take_while(|i| i * i <= n) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }

    true
}

fn is_factor(number: u64, maybe_factor: u64) -> Option<u64> {
    let rest: f64 = number as f64 / maybe_factor as f64;

    if rest.round() == rest {
        Some(rest as u64)
    } else {
        None
    }
}

fn find_factors(number: u64) -> HashSet<u64> {
    let mut maybe_factor = 2;
    let mut rest = number;
    let mut factors: HashSet<u64> = HashSet::new();

    while rest >= maybe_factor {
        if is_prime(maybe_factor) {
            if let Some(new_rest) = is_factor(number, maybe_factor) {
                rest = new_rest;
                factors.insert(maybe_factor);
            }
        }
        maybe_factor += 1;
    }

    factors
}

fn find_ghost_sleep(directions: String, map: DesertMap) -> anyhow::Result<u128> {
    let current_nodes: Vec<u64> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| find_sleep(key.clone(), &directions, &map))
        .collect::<Result<_, _>>()?;

    let mut all_factors: HashSet<u128> = HashSet::new();

    for node in current_nodes.iter() {
        for factor in find_factors(*node) {
            all_factors.insert(factor as u128);
        }
    }

    Ok(all_factors.iter().product())
}

pub fn challenge(part: u32, input: Vec<String>) -> anyhow::Result<u128> {
    let (directions, map) = parse_map(input)?;
    match part {
        1 => find_sleep("AAA".to_string(), &directions, &map).map(|it| it as u128),
        2 => find_ghost_sleep(directions, map),
        _ => Ok(0),
    }
}
