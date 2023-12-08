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

fn find_sleep(directions: String, map: DesertMap) -> anyhow::Result<u32> {
    let mut current = "AAA".to_string();
    let mut direction_index = 0;
    let mut steps = 0;

    while current != "ZZZ".to_string() {
        let (left, right) = map
            .get(&current)
            .ok_or(Error::InvalidInput(format!("{} is not in map", current)))?;
        match directions
            .chars()
            .nth(direction_index)
            .ok_or(Error::InvalidInput("How did i get here".into()))?
        {
            'R' => {
                current = right.clone();
            }
            'L' => {
                current = left.clone();
            }
            other => {
                return Err(
                    Error::InvalidInput(format!("{} SHould not be in direction", other)).into(),
                )
            }
        }
        steps += 1;
        direction_index = (direction_index + 1) % directions.len();
    }

    Ok(steps)
}

pub fn challenge(part: u32, input: Vec<String>) -> anyhow::Result<u32> {
    let (directions, map) = parse_map(input)?;
    match part {
        1 => find_sleep(directions, map),
        _ => Ok(0),
    }
}
