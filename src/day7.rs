use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use crate::Error;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err(Error::InvalidInput(format!(
                "{} is not a valid card",
                value
            ))),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut count_map: HashMap<Card, u32> = HashMap::new();
        for card in self.cards.iter() {
            let current = count_map.get(&card).unwrap_or(&0);
            count_map.insert(card.clone(), current + 1);
        }

        if count_map.len() == 1 {
            return HandType::FiveOfAKind;
        }

        if count_map.len() == 2 {
            if count_map.iter().any(|(_, count)| *count == 4) {
                return HandType::FourOfAKind;
            } else {
                return HandType::FullHouse;
            }
        }

        if count_map.len() == 3 {
            if count_map.iter().any(|(_, count)| *count == 3) {
                return HandType::ThreeOfAKind;
            } else {
                return HandType::TwoPair;
            }
        }

        if count_map.len() == 4 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').ok_or(Error::InvalidInput(format!(
            "Invalid format for hand: {}",
            s
        )))?;

        let cards = cards_str
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Card>, _>>()?;

        if cards.len() != 5 {
            return Err(Error::InvalidInput(format!(
                "Invalid amount of cards, expected 5 got: {}",
                cards.len()
            )));
        }

        Ok(Hand {
            cards,
            bid: bid_str
                .parse()
                .map_err(|err| Error::InvalidInput(format!("Unable to parse bid: {}", err)))?,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return Ordering::Equal;
        }

        let self_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();

        if self_hand_type != other_hand_type {
            return self_hand_type.cmp(&other_hand_type);
        }

        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hands<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: T,
) -> anyhow::Result<Vec<Hand>> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in input {
        hands.push(line?.parse()?);
    }
    Ok(hands)
}

fn total_winnings(hands: &mut Vec<Hand>) -> u32 {
    hands.sort();

    let mut sum: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i as u32 + 1);
    }

    sum
}

pub fn challenge<T: Iterator<Item = Result<String, std::io::Error>>>(
    input: &mut T,
) -> anyhow::Result<u32> {
    let mut hands = parse_hands(input)?;
    Ok(total_winnings(&mut hands))
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
            &mut binding
                .iter()
                .map(|it| Ok::<_, std::io::Error>(it.to_string())),
        )
        .unwrap();

        // Then
        assert_eq!(6440, total);
    }
}
