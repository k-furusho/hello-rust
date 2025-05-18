use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_str = match self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        write!(f, "{}", suit_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    suit: Suit,
    rank: u8,
}

impl Card {
    pub fn new(suit: Suit, rank: u8) -> Result<Self, &'static str> {
        if rank < 1 || rank > 13 {
            return Err("ランクは1から13の間でなければなりません");
        }
        Ok(Self { suit, rank })
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn is_ace(&self) -> bool {
        self.rank == 1
    }

    pub fn is_face_card(&self) -> bool {
        self.rank >= 11 && self.rank <= 13
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self.rank {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n => n.to_string(),
        };
        write!(f, "{}{}", self.suit, rank_str)
    }
} 