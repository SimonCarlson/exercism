#[macro_use]
extern crate maplit;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
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
    }

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
    // Split input into numbers and (1-13) and letters (S,H,D,C)
    // Count equal numbers, map to hand
    // If no 1-to-1 mapping, look for straight/high card
    let split: Vec<_> = hand.split_whitespace().collect();
    let mut numbers: Vec<u32> = Vec::new();
    let mut suites: Vec<&str> = Vec::new();
    let high_cards = hashmap!{
        "J" => 11 as u32,
        "Q" => 12 as u32,
        "K" => 13 as u32,
        "A" => 14 as u32,
    };

    for card in split {
        let mut number = card.get(..1).unwrap();
        let suite = card.get(1..).unwrap();
        let x = number.parse::<u32>();
        match x {
            Ok(v) => numbers.push(v),
            Err(_) => {
                let nr = high_cards.get(number);
                numbers.push(*nr.unwrap());
            }
        }
        suites.push(suite);
    }

    let hs: HashSet<_> = HashSet::from_iter(&numbers);
    match hs.len() {
        5 => return high_card_or_straight(&numbers, &suites),
        4 => return Hand::OnePair,
        3 => return two_pair_or_three_of_a_kind(&numbers),
        2 => return full_house_or_four_of_a_kind(&numbers),
        _ => return Hand::HighCard,
    };
    
    Hand::HighCard
}

fn high_card_or_straight(numbers: &Vec<u32>, suites: &Vec<&str>) -> Hand {
    return Hand::HighCard
}

fn two_pair_or_three_of_a_kind(numbers: &Vec<u32>) -> Hand {
    for number in numbers {
        let x = numbers.iter().filter(|v| *v == number).count();
        if x == 3 {
            return Hand::FullHouse;
        }
    }

    Hand::TwoPair
}

fn full_house_or_four_of_a_kind(numbers: &Vec<u32>) -> Hand {
    return Hand::HighCard
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
