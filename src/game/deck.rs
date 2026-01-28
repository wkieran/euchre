use super::card::{Card, Suit, Rank};
use strum::IntoEnumIterator;

pub struct Deck {
    pub cards: Vec<Card>,
}

// TODO : Deck methods
impl Deck {
    pub fn new() -> Self {
        // todo!("implement new deck");
        let mut cards = vec!();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        todo!("implement shuffle on existing deck");
    }

    fn deal(&mut self) -> Option<Card> {
        return self.cards.pop();
    }

    fn reveal_top(&self) -> Option<Card> {
        return self.cards.last().copied();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_shuffling_is_random() {
        todo!("make 10 shuffled decks and ensure none are the same");
    }
}
