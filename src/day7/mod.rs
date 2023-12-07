mod part1;
mod part2;

use std::{cmp::Ordering, str::FromStr};

use crate::Error;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Card {
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

#[derive(PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
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

fn cmp_hands<FType: Fn(&Hand) -> HandType, FCmp: Fn(&Card, &Card) -> Ordering>(
    a: &Hand,
    b: &Hand,
    hand_type: &FType,
    cmp_cards: &FCmp,
) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }

    let a_hand_type = hand_type(a);
    let b_hand_type = hand_type(b);

    if a_hand_type != b_hand_type {
        return a_hand_type.cmp(&b_hand_type);
    }

    for (a_card, b_card) in a.cards.iter().zip(b.cards.iter()) {
        match cmp_cards(a_card, b_card) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }

    Ordering::Equal
}

fn total_winnings<FType: Fn(&Hand) -> HandType, FCmp: Fn(&Card, &Card) -> Ordering>(
    hands: &mut Vec<Hand>,
    hand_type: FType,
    cmp_cards: FCmp,
) -> u32 {
    hands.sort_by(|a, b| cmp_hands(a, b, &hand_type, &cmp_cards));

    let mut sum: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i as u32 + 1);
    }

    sum
}

pub fn challenge<T: Iterator<Item = Result<String, std::io::Error>>>(
    part: u32,
    input: &mut T,
) -> anyhow::Result<u32> {
    Ok(match part {
        1 => total_winnings(&mut parse_hands(input)?, part1::hand_type, part1::cmp_cards),
        2 => total_winnings(&mut parse_hands(input)?, part2::hand_type, part2::cmp_cards),
        _ => 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_input<'a>(
        lines: &'a Vec<&str>,
    ) -> impl Iterator<Item = Result<String, std::io::Error>> + 'a {
        lines
            .iter()
            .map(|it| Ok::<_, std::io::Error>(it.to_string()))
    }

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
        let total = challenge(1, &mut to_input(&binding)).unwrap();

        // Then
        assert_eq!(6440, total);
    }

    #[test]
    fn j_ordering() {
        // Given
        let binding = vec!["J3456 1", "T345J 4"];

        // When
        let total_part1 = challenge(1, &mut to_input(&binding)).unwrap();
        let total_part2 = challenge(2, &mut to_input(&binding)).unwrap();

        // Then
        assert_eq!(4 + 2, total_part1);
        assert_eq!(1 + 8, total_part2);
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
        let total = challenge(2, &mut to_input(&binding)).unwrap();

        // Then
        assert_eq!(5905, total);
    }

    #[test]
    fn five_of_a_kind_with_jokers() {
        // Given
        let binding = vec!["JJJKJ 1", "T55J5 4"];

        // When
        let total = challenge(2, &mut to_input(&binding)).unwrap();

        // Then
        assert_eq!(2 + 4, total);
    }
}
