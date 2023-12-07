use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use crate::Error;

use super::Hand;

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

pub struct Part1Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Part1Hand {
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

impl FromStr for Part1Hand {
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

        Ok(Part1Hand {
            cards,
            bid: bid_str
                .parse()
                .map_err(|err| Error::InvalidInput(format!("Unable to parse bid: {}", err)))?,
        })
    }
}

impl PartialEq for Part1Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Part1Hand {}

impl Ord for Part1Hand {
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

impl PartialOrd for Part1Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand for Part1Hand {
    fn bid(&self) -> u32 {
        self.bid
    }
}
