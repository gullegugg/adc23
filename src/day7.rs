use std::str::FromStr;

use crate::Error;

struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').ok_or(Error::InvalidInput(format!(
            "Invalid format for hand: {}",
            s
        )))?;

        Ok(Hand {
            cards: cards_str.chars().collect(),
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
        todo!()
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
    part: u32,
    input: &mut T,
) -> anyhow::Result<u32> {
    let mut hands = parse_hands(input)?;
    match part {
        1 => Ok(total_winnings(&mut hands)),
        _ => Ok(0),
    }
}
