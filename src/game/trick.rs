use super::card::{Card, Suit, effective_suit};

#[derive(Clone, Debug)]
pub struct Trick {
    pub played_cards: Vec<(usize, Card)>, // player ID, Card played
    pub lead_suit: Option<Suit>,
}

impl Trick {
    pub fn new() -> Self {
        Trick {
            played_cards: vec![],
            lead_suit: None,
        }
    }

    pub fn play_card(&mut self, player_id: usize, card: Card, trump_suit: Suit) {
        if self.played_cards.is_empty() {
            self.lead_suit = Some(effective_suit(card, trump_suit));
        }
        let new_played_cards = (player_id, card);
        self.played_cards.push(new_played_cards);
    }

    pub fn determine_winner(&self, trump_suit: Suit) -> usize {
        let mut winner = self.played_cards[0];
        for (player_id, card) in &self.played_cards {
            if card.beats(winner.1, trump_suit, self.lead_suit.unwrap()) {
                winner = (*player_id, *card);
            }
        }
        winner.0
    }

    pub fn is_complete(&self, going_alone: bool) -> bool {
        if going_alone && self.played_cards.len() == 3 {
            return true;
        }
        if self.played_cards.len() == 4 {
            return true;
        }
        false
    }

    pub fn clear(&mut self) {
        self.played_cards.clear();
        self.lead_suit = None;
    }
}

impl Default for Trick {
    fn default() -> Self {
        Self::new()
    }
}
