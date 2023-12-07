mod part1;
mod part2;

use std::str::FromStr;

use crate::Error;

use self::{part1::Part1Hand, part2::Part2Hand};

trait Hand: Ord + FromStr<Err = Error> {
    fn bid(&self) -> u32;
}

fn parse_hands<T: Iterator<Item = Result<String, std::io::Error>>, H: Hand>(
    input: T,
) -> anyhow::Result<Vec<H>> {
    let mut hands: Vec<H> = Vec::new();
    for line in input {
        hands.push(line?.parse()?);
    }
    Ok(hands)
}

fn total_winnings<H: Hand>(hands: &mut Vec<H>) -> u32 {
    hands.sort();

    let mut sum: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid() * (i as u32 + 1);
    }

    sum
}

pub fn challenge<T: Iterator<Item = Result<String, std::io::Error>>>(
    part: u32,
    input: &mut T,
) -> anyhow::Result<u32> {
    Ok(match part {
        1 => total_winnings::<Part1Hand>(&mut parse_hands(input)?),
        2 => total_winnings::<Part2Hand>(&mut parse_hands(input)?),
        _ => 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        // Given
        let binding = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        // When
        let total = challenge(
            1,
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        // Then
        assert_eq!(6440, total);
    }

    #[test]
    fn part2_example() {
        // Given
        let binding = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        // When
        let total = challenge(
            2,
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        // Then
        assert_eq!(5905, total);
    }
}
