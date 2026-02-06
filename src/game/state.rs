use super::card::{Card, Rank, Suit};
use super::player::{Player, Team};
use super::deck::Deck;
use super::trick::Trick;

pub enum Phase {
    Dealing,
    Bidding,
    Playing,
    Scoring,
}

pub struct GameState {
    // players, deck, current trick, tricks won, trump, kitty, 
    players: [Player; 4],
    deck: Deck,
    current_trick: Trick,
    tricks_won: [usize; 2],
    trump: Option<Suit>,
    kitty: Option<Card>,
    // current phase, current player, dealer
    current_phase: Phase,
    current_player: usize,
    dealer: usize,
    // team scores, maker team
    team_scores: [usize; 2],
    maker_team: Option<Team>,
}

impl GameState {
    fn new() -> Self {
        let mut game_state = GameState{
            players: [
                Player::new(0, Team::East),
                Player::new(1, Team::West),
                Player::new(2, Team::East),
                Player::new(3, Team::West),
            ],
            deck: Deck::new(),
            current_trick: Trick::new(),
            tricks_won: [0, 0],
            trump: None,
            kitty: None,
            current_phase: Phase::Dealing,
            current_player: 0,
            dealer: 0,
            team_scores: [0, 0],
            maker_team: None,
        };
        game_state
    }

    fn controller() {
        todo!("State machine controller for the entire game");
    }

    fn new_deal(&mut self) {
        self.deck.shuffle();
        println!("test test");
        // when dealing, to make it 2-3-2-3 make +1 based on i % 2
        // 0:0-2, 1:3:4, 2:5-7, 3:8-9
        for x in 0..2 {
            for (i, player) in self.players.iter_mut().enumerate() {
                let count = if x == 0 { 2 + (i % 2) } else { 3 - (i % 2)};
                let new_cards = self.deck.deal(count).unwrap();
                player.hand.extend(new_cards);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deal(){
        let mut game_state = GameState::new();
        game_state.new_deal();
        for (i, player) in game_state.players.iter().enumerate() {
            println!("{}", player);
        }
    }
}
