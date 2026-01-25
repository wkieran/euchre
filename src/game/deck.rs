use super::card::Card;

pub struct Deck {
    cards: Vec<Card>,
}

// TODO : Deck methods
impl Deck {
    fn new() {
        todo!("implement new deck");
    }

    fn shuffle(&mut self) {
        todo!("implement shuffle on existing deck");
    }

    fn deal(&mut self) -> Option<Card> {
        todo!("implement dealing. pop card from vec");
    }

    fn reveal_top(&self) -> Card {
        todo!("flip the top card for revealing kitty card. doesn't pop from vec");
    }
}
