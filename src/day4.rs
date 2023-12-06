use std::str::FromStr;

use crate::Error;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl Card {
    fn worth(&self) -> u32 {
        let winning_count = self
            .playing_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count();

        if winning_count == 0 {
            return 0;
        }

        2_u32.pow(winning_count as u32 - 1)
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

pub fn sum_winning_cards<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: T,
) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input {
        let card: Card = line?.parse()?;
        sum += card.worth();
    }

    Ok(sum)
}

pub fn challenge<T: Iterator<Item = Result<String, std::io::Error>>>(
    part: u32,
    input: T,
) -> anyhow::Result<u32> {
    match part {
        1 => sum_winning_cards(input),
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
        let input = binding
            .iter()
            .map(|it| Ok::<_, std::io::Error>(it.to_string()));

        // When
        let sum = challenge(1, input).unwrap();

        // Then
        assert_eq!(13, sum);
    }
}
