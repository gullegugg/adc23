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

    if let Some(joker_count) = count_map.get(&Card::J).map(|it| *it) {
        count_map.remove(&Card::J);

        let mut count_vec: Vec<(&Card, &u32)> = count_map.iter().collect();

        count_vec.sort_by(|a, b| a.1.cmp(b.1));

        let highest = count_vec[count_vec.len() - 1];

        count_map.insert(highest.0.clone(), highest.1 + joker_count);
    }

    // Might be different after jokers
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
    if *a == Card::J {
        if a == b {
            return Ordering::Equal;
        } else {
            return Ordering::Less;
        }
    }

    if *b == Card::J {
        return Ordering::Greater;
    }

    a.cmp(b)
}
