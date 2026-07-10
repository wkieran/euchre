use super::card::{Card, Rank, Suit};
use rand::rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

// TODO : Deck methods
impl Deck {
    pub fn new() -> Self {
        // todo!("implement new deck");
        let mut cards = vec![];
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rng());
    }

    pub fn deal(&mut self, count: usize) -> Option<Vec<Card>> {
        if count > self.cards.len() {
            return None;
        }
        Some(self.cards.drain(..count).collect())
    }

    pub fn reveal_top(&self) -> Option<Card> {
        self.cards.last().copied()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_shuffling_is_random() {
        let mut new_deck = Deck::new();
        let before_shuffle = new_deck.cards[0];
        new_deck.shuffle();
        let after_shuffle = new_deck.cards[0];
        assert_ne!(before_shuffle, after_shuffle);
    }
}
