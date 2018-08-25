use std::collections::HashSet;
use std::iter::FromIterator;

// I found this exercise tricky, looked a lot at shybyte's solution

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hands: Vec<_> = hands.iter().map(|hand| (hand, Hand::from(hand).hand_type())).collect();

    None
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn from (hand_string: &str) -> Hand {
        let cards = hand_string.split(" ").map(Card::from).collect();

        Hand { cards }
    }

    fn hand_type(&self) -> HandType {
        let mut ranks = Vec::new();
        let mut suites = Vec::new();
        
        for card in &self.cards {
            ranks.push(&card.rank);
            suites.push(&card.suite);
        }

        let rank_hs: HashSet<_> = HashSet::from_iter(ranks.iter());
        let suite_hs: HashSet<_> = HashSet::from_iter(suites.iter());
        let is_straight;
        if suite_hs.len() == 5 {
            is_straight = true;
        } else {
            is_straight = false;
        }

        match rank_hs.len() {
            5 => {
                
            },
            4 => {

            },
            3 => {

            },
            2 => {

            },
            1 => {
                
            },
            _ => panic!("No cards in hand.")
        }


        HandType::HighCard
    }

}

struct Card {
    rank: Rank,
    suite: Suite,
}

impl Card {
    fn from(card_string: &str) -> Card {
        let (head, tail) = card_string.split_at(card_string.len() - 1);
        Card {
            rank: Rank::from(head),
            suite: Suite::from(tail)
        }
    }
}

enum Suite {
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

impl Suite {
    fn from(suite: &str) -> Suite {
        match suite {
            "C" => Suite::CLUBS,
            "D" => Suite::DIAMONDS,
            "H" => Suite::HEARTS,
            "S" => Suite::SPADES,
            _ => panic!("Invalid suite")
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Rank(u32);

impl Rank {
    fn from(rank: &str) -> Rank {
        match rank {
            "A" => Rank(14),
            "K" => Rank(13),
            "Q" => Rank(12),
            "J" => Rank(11),
            v => Rank(v.parse().unwrap())
        }
    }
}


#[derive(PartialOrd, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush
}