use std::collections::HashMap;

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

fn find_ghost_sleep(directions: String, map: DesertMap) -> anyhow::Result<u64> {
    let current_nodes: Vec<u64> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| find_sleep(key.clone(), &directions, &map))
        .collect::<Result<_, _>>()?;

    Ok(current_nodes.iter().fold(1, |a, b| num_integer::lcm(a, *b)))
}

pub fn challenge(part: u32, input: Vec<String>) -> anyhow::Result<u64> {
    let (directions, map) = parse_map(input)?;
    match part {
        1 => find_sleep("AAA".to_string(), &directions, &map),
        2 => find_ghost_sleep(directions, map),
        _ => Ok(0),
    }
}
