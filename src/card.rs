use strum::{IntoEnumIterator, EnumIter, Display};

#[derive(EnumIter, Copy, Clone, PartialEq, Display)]
pub enum Suit {
    Hearts, Spades, Clubs, Diamonds
}

#[derive(EnumIter, Copy, Clone, PartialEq, Display)]
pub enum Rank {
    Ace,
    Two = 2, Three = 3, Four = 4, Five = 5, Six = 6,
    Seven = 7, Eight = 8, Nine = 9, Ten = 10,
    Jack, Queen, King,
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub value: u8,
    pub is_hole_card: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank, value: u8) -> Self {
        Self { suit, rank, value, is_hole_card: false }
    }

    pub fn get_name(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }
}