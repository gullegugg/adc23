use std::str::FromStr;

use crate::Error;

struct Set {
    blue: u32,
    green: u32,
    red: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id_str, sets) = line
            .split_once(':')
            .ok_or(Error::InvalidInput("Missing :".to_string()))?;
        let id: u32 = id_str
            .split_once(' ')
            .ok_or(Error::InvalidInput(
                "Missing whitespace in game id".to_string(),
            ))?
            .1
            .parse()?;

        let mut parsed_sets: Vec<Set> = Vec::new();
        for set in sets.split(';') {
            let mut parsed_set = Set {
                blue: 0,
                green: 0,
                red: 0,
            };
            for cube_count in set.split(',').map(|it| it.trim()) {
                let (count_str, color) = cube_count.split_once(' ').ok_or(Error::InvalidInput(
                    "Missing whitespace in cube count".to_string(),
                ))?;
                let count: u32 = count_str.parse()?;
                match color {
                    "blue" => {
                        parsed_set.blue = count;
                    }
                    "red" => {
                        parsed_set.red = count;
                    }
                    "green" => {
                        parsed_set.green = count;
                    }
                    _ => Err(Error::InvalidInput(
                        "Invalid color in cube count".to_string(),
                    ))?,
                }
            }
            parsed_sets.push(parsed_set);
        }
        Ok(Game {
            id,
            sets: parsed_sets,
        })
    }
}

fn parse_games(lines: Vec<String>) -> impl Iterator<Item = anyhow::Result<Game>> {
    lines.into_iter().map(|line| line.parse())
}

fn sum_valid_ids<Iter: Iterator<Item = anyhow::Result<Game>>>(games: Iter) -> anyhow::Result<u32> {
    let mut sum = 0;
    for game_res in games {
        let game = game_res?;
        if game
            .sets
            .iter()
            .all(|set| set.blue <= 14 && set.green <= 13 && set.red <= 12)
        {
            sum += game.id
        }
    }
    Ok(sum)
}

fn max_by<F: Fn(&Set) -> u32>(sets: &Vec<Set>, by_key: F) -> u32 {
    sets.iter()
        .map(by_key)
        .max()
        .map(|min| if min == 0 { 1 } else { min })
        .unwrap_or(1)
}

fn power_of_min<Iter: Iterator<Item = anyhow::Result<Game>>>(games: Iter) -> anyhow::Result<u32> {
    let mut sum = 0;
    for game_res in games {
        let game = game_res?;
        let red = max_by(&game.sets, |set| set.red);
        let blue = max_by(&game.sets, |set| set.blue);
        let green = max_by(&game.sets, |set| set.green);

        sum += red * green * blue;
    }

    Ok(sum)
}

pub fn elf_challenge(part: u32, lines: Vec<String>) -> anyhow::Result<u32> {
    let games = parse_games(lines);

    Ok(match part {
        1 => sum_valid_ids(games)?,
        2 => power_of_min(games)?,
        _ => 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // Given
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines()
            .map(|it| it.to_string())
            .collect();

        // When
        let sum = elf_challenge(1, input).unwrap();

        // Then
        assert_eq!(1 + 2 + 5, sum);
    }

    #[test]
    fn part2_example() {
        // Given
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines()
            .map(|it| it.to_string())
            .collect();

        // When
        let sum = elf_challenge(2, input).unwrap();

        // Then
        assert_eq!(2286, sum);
    }
}
