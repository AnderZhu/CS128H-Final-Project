use super::*;
use crate::black_jack::card::Card;
use crate::black_jack::deck::Deck;

pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}

// pub struct Dealer {
//     pub name: String,
//     hand: Vec<Card>,
// }

impl Player {
    fn new(name: String) -> Player {
        let mut hand = vec![];
        Player { name, hand }
    }

    // fn deal_card(&mut self, card: Card);
    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_score(&self) -> u32 {
        //get score of all cards added
        let mut score = 0;
        score = self.hand.iter().fold(0, |acc, card| acc + card.score());

        // situation when might want A as 11 or 1
        if score > 21 {
            for card in &self.hand {
                if card.value.as_str() == "A" {
                    score -= 10;
                    if score <= 21 {
                        break;
                    }
                }
            }
        }
        score
    }

    //determine if player has black_jack
    fn has_black_jack(&self) -> bool {
        return self.get_score() == 21;
    }

    //initial round get from deck
    fn initial_r(&mut self, deck: &mut Deck) {}

    //deal or not
    fn get_card(&mut self, deck: &mut Deck) {
        let card = deck.deal_card();
        self.hand.push(card);
    }

    fn next_move() {}
}
