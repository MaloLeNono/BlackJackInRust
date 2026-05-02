use crate::card::{Card, Rank};
use crate::deck::Deck;

pub struct Hand {
    pub cards: Vec<Card>,
    name: String
}

impl Hand {
    pub fn new(deck: &mut Deck, name: String) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        cards.append(&mut deck.pull_cards(2));
        Self {cards, name}
    }

    pub fn get_sum(&self) -> u8 {
        let mut sum: u8 = 0;
        for card in &self.cards {
            if card.rank == Rank::Jack || card.rank == Rank::Queen || card.rank == Rank::King {
                sum += 10;
                continue;
            }

            if card.rank == Rank::Ace {
                if sum + 11 > 21{
                    sum += 1;
                } else {
                    sum += 11;
                }
                continue;
            }

            sum += card.value;
        }

        sum
    }

    pub fn print(&self, show_hole_card: bool) {
        println!("====================\n");

        let mut content = String::new();
        content.push_str(&format!("{}:\n", self.name));

        let mut contains_hole_card = false;
        for card in &self.cards {
            if card.is_hole_card {
                contains_hole_card = true;
                break;
            }
        }

        let hand_sum = format!("Sum: {},\n", self.get_sum());
        let sum: String = match contains_hole_card {
            true => {
                if show_hole_card {
                    hand_sum
                } else {
                    String::from("Sum: ?,\n")
                }
            }
            false => {
                hand_sum
            }
        };


        content.push_str(&sum);

        for card in &self.cards {
            if !card.is_hole_card || show_hole_card {
                content.push_str(&format!("{}\n", card.get_name()));
            } else {
                content.push_str(&"-- HOLE CARD --\n");
            }
        }

        println!("{}", content);
        println!("====================");
    }
}