use super::card::{Card, Rank, Suit, effective_suit};
use super::player::{Player, Team};
use super::deck::Deck;
use super::trick::Trick;

// === Enums ===
#[derive(PartialEq, Debug)]
pub enum Phase {
    Dealing,
    Bidding,
    Discarding,
    StuckDealer,
    Playing,
    Scoring,
    GameOver,
}
enum BidAction {
    Pass,
    OrderUp,
    OrderUpAlone,
    CallSuit(Suit),
    CallSuitAlone(Suit),
}

// === Structs ===
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
    // bidding state
    bidding_round: usize,
    current_bidder: usize,
    bids_passed: usize,
}

// === Impl Blocks ===
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
            bidding_round: 0,
            current_bidder: 0,
            bids_passed: 0,
        };
        game_state
    }

    // --- dealing ---
    fn new_deal(&mut self) {
        self.deck.shuffle();
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

    // --- bidding ---
    fn start_bidding(&mut self) {
        self.kitty = self.deck.reveal_top();
        self.bidding_round = 1;
        self.current_bidder = (self.dealer + 1) % 4;
    }

    fn submit_bid(&mut self, bid: BidAction) {
        match bid {
            BidAction::Pass => {
                self.bids_passed += 1;
                if self.bids_passed == 4 && self.bidding_round == 1 {
                    self.bids_passed = 0;
                    self.bidding_round = 2;
                } else if self.bids_passed == 4 && self.bidding_round == 2 {
                    self.current_phase = Phase::StuckDealer;
                }
                self.current_bidder = (self.current_bidder + 1) % 4;
            },
            BidAction::OrderUp | BidAction::OrderUpAlone => {
                assert_eq!(self.bidding_round, 1, "cannot order up in round 2!!");
                self.trump = Some(self.kitty.unwrap().suit);
                self.set_maker_and_adavance(matches!(bid, BidAction::OrderUpAlone));
                self.dealer_pickup();

            },
            BidAction::CallSuit(suit) | BidAction::CallSuitAlone(suit) => {
                assert_eq!(self.bidding_round, 2, "Cannot call suit in round 1!!");
                assert_ne!(suit, self.kitty.unwrap().suit, "Cannot call the same suit as Kitty!!");
                self.trump = Some(suit);
                self.set_maker_and_adavance(matches!(bid, BidAction::CallSuitAlone(_)));
                self.current_phase = Phase::Playing;
            },
        }
    }

    fn set_maker_and_adavance(&mut self, going_alone: bool) {
        self.maker_team = Some(if self.current_bidder % 2 == 0 { Team::East } else { Team::West });
        if going_alone {
            self.players[self.current_bidder].is_going_alone = true;
        }
    }

    fn dealer_pickup(&mut self) {
        // self.players[self.dealer].hand.extend(vec![self.kitty]);
        self.players[self.dealer].hand.push(self.kitty.unwrap());
        self.current_phase = Phase::Discarding;
    }

    fn submit_discard(&mut self, card_index: usize) {
        assert_eq!(self.current_phase, Phase::Discarding);
        self.players[self.dealer].hand.remove(card_index);
        assert_eq!(self.players[self.dealer].hand.len(), 5);
        self.current_phase = Phase::Playing;
    }

    fn submit_stuck_call(&mut self, suit: Suit) {
        assert_eq!(self.current_phase, Phase::StuckDealer);
        assert_ne!(suit, self.kitty.unwrap().suit);
        self.trump = Some(suit);
        self.set_maker_and_adavance(false);
        self.current_phase = Phase::Playing;
    }

    // --- playing ---
    fn next_player(&mut self, from: usize) {
    // Find who's sitting out: partner of the player going alone
    let sitting_out = self.players.iter().position(|p| p.is_going_alone).map(|n| (n + 2) % 4);

    let mut next = (from + 1) % 4;
    if sitting_out == Some(next) {
        next = (from + 2) % 4;
    }
    self.current_player = next;
}

    fn play_card(&mut self, player_id: usize, card_index: usize) {
        assert_eq!(self.current_phase, Phase::Playing);
        assert_eq!(player_id, self.current_player);

        let card_to_play = self.players[player_id].hand[card_index];

        if self.current_trick.played_cards.len() > 0 {
            for (i, card) in self.players[player_id].hand.iter().enumerate() {
                if effective_suit(*card, self.trump.unwrap()) != self.current_trick.lead_suit.unwrap() && i == card_index {
                    panic!("Player is trying to reneg!");
                }
            }
        }
        
        self.players[player_id].hand.remove(card_index);
        self.current_trick.play_card(player_id, card_to_play, self.trump.unwrap());
    }

    fn handle_trick(&mut self) {
        if !self.current_trick.is_complete(self.players.iter().any(|p| p.is_going_alone)) {
            self.next_player(self.current_player);
        }
        else {
            let winner = self.current_trick.determine_winner(self.trump.unwrap());
            let winning_team = winner % 2;
            self.tricks_won[winning_team] += 1;
            self.current_trick.clear();
            let tricks_played = self.tricks_won[0] + self.tricks_won[1];
            if tricks_played == 5 {
                self.current_phase = Phase::Scoring;
            } else {
                self.current_player = winner;
            }
        }
    }
    
    // --- scoring ---
    // count tricks, score points
    fn score_round(&mut self) {
        let maker = self.maker_team.unwrap() as usize;
        let defender = 1 - maker;
        let maker_wins = self.tricks_won[maker];
        let defender_wins = self.tricks_won[defender];

        match maker_wins {
            5 => {
                if self.players.iter().any(|p| p.is_going_alone && p.team as usize == maker) {
                    self.team_scores[maker] += 4;
                } else {
                    self.team_scores[maker] += 2;
                }
            },
            3..=4 => {
                self.team_scores[maker] += 1;
            },
            0..=2 => {
                self.team_scores[defender] += 2;
            }
            _ => unreachable!()
        }
        if self.team_scores[0] >= 10 || self.team_scores[1] >= 10 {
            self.current_phase = Phase::GameOver;
        } else {
            self.dealer = (self.dealer + 1) % 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deal() {
        let mut game_state = GameState::new();
        game_state.new_deal();
        for (i, player) in game_state.players.iter().enumerate() {
            println!("{}", player);
            assert_eq!(player.hand.len(), 5);
        }
        assert_eq!(game_state.deck.cards.len(), 4);
        // TODO : add a test for unique player hands. it's just printed rn which is p good.
    }

    #[test]
    fn test_submit_bid() {
        let mut game_state = GameState::new();
        game_state.new_deal();
        game_state.start_bidding();
        assert!(game_state.kitty.is_some());
        assert!(game_state.bidding_round == 1);
        assert!(game_state.current_bidder == 1);

        // TODO : finish testing submit_bid()
        //
        // game_state.submit_bid(BidAction::Pass);
        // game_state.submit_bid(BidAction::Pass);
        // game_state.submit_bid(BidAction::Pass);
        // game_state.submit_bid(BidAction::Pass);
        // game_state.submit_bid(BidAction::Pass);
        // assert_eq!(bids_passed, 5);
        // assert_eq!(current_bidder, 6);
        //
        // game_state.submit_bid(BidAction::CallSuit(Suit::Hearts));
        //
    }
    // TODO : 
    // - test_submit_discard()
    // - test entire bidding phase stuff
}
