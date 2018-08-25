/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {

    None
}

struct Hand {
    cards: Vec<Card>,
    score: u32,
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