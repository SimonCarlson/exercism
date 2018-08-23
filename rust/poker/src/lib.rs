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
        println!("{}", hand);
        let hand_type = find_hand_type(hand);
        hm.insert(hand, hand_type);
    }

    let hand_types: Vec<_> = hm.iter().collect();

    let mut winners: Vec<&str> = Vec::new();

    let max = hand_types.iter().max_by(|x, y| x.1.partial_cmp(y.1).unwrap()).unwrap();
    //println!("{:?}", max.1);

    for winner in &hand_types {
        if winner.1 == max.1 {
            winners.push(winner.0);
        }
    }

    return Some(winners)
}

pub fn find_hand_type(hand: &str) -> Hand {
    // Split input into numbers and (1-13) and letters (S,H,D,C)
    // Count equal numbers, map to hand types
    let mut numbers: Vec<u32> = Vec::new();
    let mut suites: Vec<&str> = Vec::new();
    let high_cards = hashmap!{
        "J" => 11 as u32,
        "Q" => 12 as u32,
        "K" => 13 as u32,
        "A" => 14 as u32,
    };

    let split: Vec<_> = hand.split_whitespace().collect();
    for card in split {
        let mut number;
        let mut suite = card.get(1..).unwrap();
        if suite.len() > 1 {
            suite = card.get(2..).unwrap();
            number = card.get(..2).unwrap();
        } else {
            number = card.get(..1).unwrap();
        }

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


    // Find if normal flush (same suites)

    let hs: HashSet<_> = HashSet::from_iter(&numbers);
    println!("{}", hs.len());
    match hs.len() {
        5 => return high_card_or_straight(&numbers, &suites),
        4 => return Hand::OnePair,
        3 => return two_pair_or_three_of_a_kind(&numbers),
        2 => return full_house_or_four_of_a_kind(&numbers),
        _ => return Hand::HighCard,
    };
}

fn high_card_or_straight(numbers: &Vec<u32>, suites: &Vec<&str>) -> Hand {
    // If cards are in sequence, straight
    // If all same suites, straight flush
    // Else high card

    for (low, high) in numbers.iter().zip(numbers.iter().skip(1)) {
           // If any pair differs by more than one, it is not a straight
           if high - low > 1 {
               return Hand::HighCard
           }
    }

    // If it is a straight, is it also a flush?
    if is_flush(suites) {
        return Hand::StraightFlush
    } else {
        return Hand::Straight
    }
}

fn is_flush(suites: &Vec<&str>) -> bool {
    let hs: HashSet<_> = HashSet::from_iter(suites.iter());
    hs.len() == 1
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
    for number in numbers {
        let x = numbers.iter().filter(|v| *v == number).count();
        if x == 4 {
            return Hand::FourOfAKind
        }
    }

    Hand::FullHouse
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}
