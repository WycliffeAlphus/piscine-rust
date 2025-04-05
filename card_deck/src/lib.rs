use std::time::{SystemTime, UNIX_EPOCH};

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
        let seed = current_time_seed();
        let value = (seed % 4 + 1) as u8; // 1..=4
        Suit::translate(value).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8), // 2 to 10
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
        let seed = current_time_seed();
        let value = (seed % 13 + 1) as u8; // 1..=13
        Rank::translate(value).unwrap()
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

/// Get a pseudo-random seed from current system time
fn current_time_seed() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.subsec_nanos() as u64 + now.as_secs()
}
