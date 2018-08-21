use std::collections::HashMap;
use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hm: HashMap<&str, Hand> = HashMap::new();
    for &hand in hands.iter() {
        let hand_type = find_hand_type(hand);
        hm.insert(hand, hand_type);
    };

    let mut hand_types: Vec<_> = hm.iter().collect();
    hand_types.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Greater));

    let mut winning_type = &Hand::HighCard;
    let mut winners: Vec<&str> = Vec::new();
    for winner in hand_types {
        if winner.1 >= winning_type {
            winning_type = winner.1;
            winners.push(winner.0);
        }
    }

    return Some(winners)
}

pub fn find_hand_type(hand: &str) -> Hand {
    Hand::HighCard
}

#[derive(PartialOrd, PartialEq)]
pub enum Hand {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
