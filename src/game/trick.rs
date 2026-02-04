use super::card::{Card, Suit, effective_suit};

pub struct Trick {
    played_cards: Vec<(usize, Card)>, // player ID, Card played
    lead_suit: Option<Suit>,
}

impl Trick {
    pub fn new() -> Self {
        let mut trick = Trick {
            played_cards: vec!(),
            lead_suit: None,
        };
        trick
    }

    fn play_card(&mut self, player_id: usize, card: Card, trump_suit: Suit) {
        if self.played_cards.len() == 0 {
            self.lead_suit = Some(effective_suit(card, trump_suit));
        }
        let new_played_cards = (player_id, card);
        self.played_cards.push(new_played_cards);

    }

    fn determine_winner(&self, trump_suit: Suit) -> usize {
       let mut winner = self.played_cards[0];
       for (player_id, card) in &self.played_cards {
           if card.beats(winner.1, trump_suit, self.lead_suit.unwrap()) {
               winner = (*player_id, *card);
           }
       }
       winner.0
    }

    fn is_complete(&self, going_alone: bool) -> bool {
        if going_alone && self.played_cards.len() == 3 {
            return true;
        }
        if self.played_cards.len() == 4 {
            return true;
        }
        false
    }

    fn clear(&mut self) {
        self.played_cards.clear();
        self.lead_suit = None;
    }
}

