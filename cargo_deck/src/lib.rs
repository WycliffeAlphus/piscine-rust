use rand::rng;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn translate(value: u8) -> Option<Suit> {
        match value {
            1 => Some(Suit::Heart),
            2 => Some(Suit::Diamond),
            3 => Some(Suit::Spade),
            4 => Some(Suit::Club),
            _ => None,
        }
    }

    pub fn random() -> Suit {
        let n = rng().random_range(1..=4);
        Suit::translate(n).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8), // 2..=10
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn translate(value: u8) -> Option<Rank> {
        match value {
            1 => Some(Rank::Ace),
            2..=10 => Some(Rank::Number(value)),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            _ => None,
        }
    }

    pub fn random() -> Rank {
        let n = rng().random_range(1..=13);
        Rank::translate(n).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
