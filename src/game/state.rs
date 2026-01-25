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
