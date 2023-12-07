use std::{collections::HashMap, str::FromStr};

use crate::Error;

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl Card {
    fn winning_count(&self) -> u32 {
        let winning_count = self
            .playing_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count();

        winning_count as u32
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_prefix, num_part) = s
            .split_once(':')
            .ok_or(Error::InvalidInput("Missing ':' in Card input".to_string()))?;

        let (winning, playing) = num_part
            .split_once('|')
            .ok_or(Error::InvalidInput("Missing '|' in Card input".to_string()))?;

        let winning_numbers = winning
            .trim()
            .split(' ')
            .filter(|str_num| !str_num.is_empty())
            .map(|str_num| str_num.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| {
                Error::InvalidInput(format!("Failed to parse winning numbers: {}", err))
            })?;

        let playing_numbers = playing
            .trim()
            .split(' ')
            .filter(|str_num| !str_num.is_empty())
            .map(|str_num| str_num.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| {
                Error::InvalidInput(format!("Failed to parse playing numbers: {}", err))
            })?;

        Ok(Card {
            playing_numbers,
            winning_numbers,
        })
    }
}

fn sum_winning_cards(input: Vec<String>) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input {
        let card: Card = line.parse()?;
        let winning_count = card.winning_count();
        if winning_count > 0 {
            sum += 2_u32.pow(winning_count - 1)
        }
    }

    Ok(sum)
}

fn count_winning_cards(input: Vec<String>) -> anyhow::Result<u32> {
    let mut card_amount_map: HashMap<usize, u32> = HashMap::new();
    let mut num_cards = 0;
    for (i, line) in input.iter().enumerate() {
        let card: Card = line.parse()?;
        let winning_count = card.winning_count();
        let card_count = card_amount_map.get(&i).unwrap_or(&0) + 1;

        card_amount_map.insert(i, card_count);

        for winning_offset in 1..winning_count + 1 {
            let index_to_update = i + winning_offset as usize;
            let current_count = card_amount_map.get(&index_to_update).unwrap_or(&0);

            card_amount_map.insert(index_to_update, current_count + card_count);
        }
        num_cards += 1;
    }

    Ok(card_amount_map
        .iter()
        .filter(|(i, _)| **i < num_cards as usize)
        .map(|(_, v)| v)
        .sum())
}

pub fn challenge(part: u32, input: Vec<String>) -> anyhow::Result<u32> {
    match part {
        1 => sum_winning_cards(input),
        2 => count_winning_cards(input),
        _ => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // Given
        let binding = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let input = binding.iter().map(|line| line.to_string()).collect();

        // When
        let sum = challenge(1, input).unwrap();

        // Then
        assert_eq!(13, sum);
    }

    #[test]
    fn part2_example() {
        // Given
        let binding = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let input = binding.iter().map(|line| line.to_string()).collect();

        // When
        let sum = challenge(2, input).unwrap();

        // Then
        assert_eq!(30, sum);
    }
}
