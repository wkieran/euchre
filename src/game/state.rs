use super::card::{Card, Rank, Suit};
use super::player::{Player, Team};
use super::deck::Deck;
use super::trick::Trick;

// === Enums ===
pub enum Phase {
    Dealing,
    Bidding,
    Playing,
    Scoring,
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
                self.current_bidder = (self.current_bidder + 1) % 4;
                if self.bids_passed == 4 && self.bidding_round == 1 {
                    self.bids_passed = 0;
                    self.bidding_round = 2;
                } else if self.bids_passed == 3 && self.bidding_round == 2 {
                    // TODO : dealer must call!!
                }
            },
            BidAction::OrderUp => {
                if self.bidding_round == 2 {
                    panic!("cannot order up in round 2!!");
                }
                self.trump = Some(self.kitty.unwrap().suit);
                self.maker_team = if self.current_bidder % 2 == 0 { Some(Team::East) } else { Some(Team::West) };
                self.dealer_pickup();
                self.current_phase = Phase::Playing;
            },
            BidAction::OrderUpAlone => {
                if self.bidding_round == 2 {
                    panic!("cannot order up in round 2!!");
                }
                self.trump = Some(self.kitty.unwrap().suit);
                self.maker_team = if self.current_bidder % 2 == 0 { Some(Team::East) } else { Some(Team::West) };
                self.dealer_pickup();
                self.current_phase = Phase::Playing;
                self.players[self.current_bidder].is_going_alone = true;
            },
            BidAction::CallSuit(suit) => {
                if self.bidding_round == 1 {
                    panic!("cannot call suit in round 1!!");
                }
                if suit == self.kitty.unwrap().suit {
                    panic!("cannot call same suit as kitty!!");
                }

                self.trump = Some(suit);
                self.maker_team = if self.current_bidder % 2 == 0 { Some(Team::East) } else { Some(Team::West) };
                self.current_phase = Phase::Playing
            },
            BidAction::CallSuitAlone(suit) => {
                if self.bidding_round == 1 {
                    panic!("cannot call suit in round 1!!");
                }
                if suit == self.kitty.unwrap().suit {
                    panic!("cannot call same suit as kitty!!");
                }

                self.trump = Some(suit);
                self.maker_team = if self.current_bidder % 2 == 0 { Some(Team::East) } else { Some(Team::West) };
                self.current_phase = Phase::Playing;
                self.players[self.current_bidder].is_going_alone = true;
            },
        }
    }

    fn dealer_pickup(&mut self) {
        // self.players[self.dealer].hand.extend(vec![self.kitty]);
        self.players[self.dealer].hand.extend(self.deck.deal(1).unwrap()); // TODO maybe use
                                                                           // .take() instead of
                                                                           // extend
        // TODO : dealer discards one card
    }

    // --- playing ---
    // loop through 5 tricks, track who wins each
    
    // --- scoring ---
    // count tricks, score points

    // --- controller ---
    fn controller() {
        todo!("State machine controller for the entire game");
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
}
