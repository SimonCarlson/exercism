use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hm = HashMap::new();
    for hand in hands.iter() {
        let hand_type = find_hand_type(hand);
        hm.insert(hand, hand_type);
    };

    return None
}

pub fn find_hand_type(hand: &&str) -> Hand {
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
