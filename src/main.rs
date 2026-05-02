mod card;
mod deck;
mod hand;

use std::io::Read;
use crate::deck::Deck;
use crate::hand::Hand;

fn end_game(message: &str, player_hand: &Hand, dealer_hand: &Hand) {
    println!("\n{}\n", message);
    player_hand.print(true);
    dealer_hand.print(true);
}

fn main() {
    let mut deck = Deck::new();
    let mut player_hand = Hand::new(&mut deck, String::from("Your Hand"));
    let mut dealer_hand = Hand::new(&mut deck, String::from("Dealer's Hand"));
    dealer_hand.cards[0].is_hole_card = true;
    player_hand.print(false);
    dealer_hand.print(false);

    let mut running = true;
    let mut handle = std::io::stdin();
    let mut input = String::new();
    while running {
        input.clear();
        println!("What would you like to do? (hit/stand)");
        _ = handle.read_line(&mut input);

        println!();
        match input.to_lowercase().as_str().trim() {
            "hit" => {
                player_hand.cards.append(&mut deck.pull_cards(1));
                if player_hand.get_sum() > 21 {
                    end_game("-- BUST! --", &player_hand, &dealer_hand);
                    running = false;
                    continue;
                }

                player_hand.print(false);
                dealer_hand.print(false);
            }
            "stand" => {
                while dealer_hand.get_sum() <= 17 {
                    player_hand.print(true);
                    dealer_hand.print(true);
                    dealer_hand.cards.append(&mut deck.pull_cards(1));
                }

                let dealer_sum = dealer_hand.get_sum();
                let player_sum = player_hand.get_sum();
                if dealer_sum > 21 {
                    end_game("-- WIN! --", &player_hand, &dealer_hand);
                    running = false;
                    continue;
                } else if dealer_sum < player_sum {
                    end_game("-- WIN! --", &player_hand, &dealer_hand);
                    running = false;
                    continue;
                } else if dealer_sum > player_sum {
                    end_game("-- LOSE! --", &player_hand, &dealer_hand);
                    running = false;
                    continue;
                } else if dealer_sum == player_sum {
                    end_game("-- PUSH! --", &player_hand, &dealer_hand);
                    running = false;
                    continue;
                }
            }
            _ => {
                println!("Type either hit or stand.");
            }
        }
    }

    println!("Press enter to exit...");
    let _ = handle.read(&mut [0u8]).unwrap();
}