use std::collections::HashSet;
use std::iter::FromIterator;

// I found this exercise tricky, looked a lot at shybyte's solution

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    // Store the hand ref with its type
    // Sort the list after type, descending
    // Pick all hands of the highest type as winners? Or some cost function?
    let mut hands: Vec<_> = hands.iter().map(|hand| (hand, Hand::from(hand).hand_type())).collect();
    hands.sort_by(|&(_, ref type1), &(_, ref type2)| type2.cmp(type1));
    let winning_type = hands[0].1.clone();

    let winners: Vec<&'a str> = hands.into_iter()
        .take_while(|&(_, ref hand_type)| *hand_type == winning_type)
        .map(|(&hand_string, _)| hand_string)
        .collect();
    
    Some(winners)
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
        let is_flush;
        if suite_hs.len() == 1 {
            is_flush = true;
        } else {
            is_flush = false;
        }

        // Determine what kind of hand it is
        let hand_type;
        match rank_hs.len() {
            5 => {
                hand_type = high_card_or_straight(&self.cards)
            },
            4 => {
                hand_type = HandType::OnePair
            },
            3 => {
                hand_type = two_pair_or_three_of_a_kind(&self.cards)
            },
            2 => {
                hand_type = full_house_or_four_of_a_kind(&self.cards)
            },
            _ => hand_type = HandType::HighCard([ranks[0].0,ranks[1].0,ranks[2].0,ranks[3].0,ranks[4].0])
        }

        if let HandType::HighCard([_, _, _, _, _]) = hand_type {
            if is_flush {
                return HandType::Flush
            }
        }

        if let HandType::Straight = hand_type {
            if is_flush {
                return HandType::StraightFlush
            }
        }

        hand_type
    }

}

fn high_card_or_straight(cards: &Vec<Card>) -> HandType {
    let mut ranks: Vec<_> = Vec::new();
    for card in cards {
        ranks.push(&card.rank.0)
    }

    for (&low, &high) in ranks.iter().zip(ranks.iter().skip(1)) {
        if high - low > 1 {
            return HandType::HighCard([*ranks[0],*ranks[1],*ranks[2],*ranks[3],*ranks[4]])
        }
    }

    HandType::Straight
}

fn two_pair_or_three_of_a_kind(cards: &Vec<Card>) -> HandType {
    let mut ranks: Vec<_> = Vec::new();
    for card in cards {
        ranks.push(&card.rank.0)
    }

    for rank in &ranks {
        let x = ranks.iter().filter(|v| *v == rank).count();
        if x == 3 {
            return HandType::ThreeOfAKind
        }
    }

    HandType::TwoPair
}

fn full_house_or_four_of_a_kind(cards: &Vec<Card>) -> HandType {
    let mut ranks: Vec<_> = Vec::new();
    for card in cards {
        ranks.push(&card.rank.0)
    }

    for rank in &ranks {
        let x = ranks.iter().filter(|v| *v == rank).count();
        if x == 4 {
            return HandType::FourOfAKind
        }
    }

    HandType::FullHouse
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

#[derive(Hash, Eq, PartialEq)]
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

#[derive(Hash, Eq, PartialEq, Debug, Clone, Ord, PartialOrd)]
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


#[derive(PartialOrd, PartialEq, Ord, Eq, Clone)]
enum HandType {
    HighCard([u32; 5]),
    OnePair([u32; 5]),
    TwoPair([u32; 5]),
    ThreeOfAKind([u32; 5]),
    Straight([u32; 5]),
    Flush([u32; 5]),
    FullHouse([u32; 5]),
    FourOfAKind([u32; 5]),
    StraightFlush([u32; 5])
}