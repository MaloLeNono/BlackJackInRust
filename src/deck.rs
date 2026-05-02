use rand::prelude::ThreadRng;
use rand::RngExt;
use strum::IntoEnumIterator;
use crate::card::{Card, Rank, Suit};

pub struct Deck {
    pub cards: Vec<Card>,
    rng: ThreadRng
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        let rng = rand::rng();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank, rank as u8))
            }
        }

        Self {cards, rng}
    }

    pub fn pull_cards(&mut self, amount: u8) -> Vec<Card> {
        let mut cards_to_return: Vec<Card> = Vec::new();
        for _ in 0..amount {
            let random_index = self.rng.random_range(0..self.cards.len());
            let card = self.cards.swap_remove(random_index);
            cards_to_return.push(card);
        }

        cards_to_return
    }
}