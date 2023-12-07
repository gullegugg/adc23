use std::{cmp::Ordering, collections::HashMap};

use super::{Card, Hand, HandType};

pub fn hand_type(hand: &Hand) -> HandType {
    let mut count_map: HashMap<Card, u32> = HashMap::new();
    for card in hand.cards.iter() {
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

pub fn cmp_cards(a: &Card, b: &Card) -> Ordering {
    a.cmp(b)
}
