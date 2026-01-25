use super::card::{Card, Suit};

pub struct Trick {
    played_cards: Vec<(usize, Card)>, // player ID, Card played
    lead_suit: Suit,
    trump_suit: Suit,
}

impl Trick {
    fn play_card(&self) {
        todo!("implement player play card in trick if it's their turn");
    }

    fn determine_winner() {
        todo!("determine player winner of trick if trick is over");
    }

    fn is_complete() {
        todo!("determine if trick is over");
    }

    fn clear() {
        todo!("prepare for next trick");
    }
}

