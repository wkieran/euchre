// src/game/controller.rs
use super::state::{GameState, Phase, BidAction};
use super::card::{Card, Suit};
use super::player::{Team};

pub enum GameAction {
    //Bidding
    Pass,
    OrderUp,
    OrderUpAlone,
    CallSuit(Suit),
    CallSuitAlone(Suit),
    //Discarding
    Discard(usize),
    //Stuck (dealer)
    StuckCall(Suit),
    //Playing
    PlayCard(usize),
}

pub enum GameEvent {
    NewHand,
    BiddingStarted { kitty: Card },
    TrickComplete { winner: usize },
    HandComplete { scores: [usize; 2] },
    GameOver { winner: Team},
    PhaseChanged(Phase),
}

pub struct GameController {
    pub state: GameState,
}

impl GameController {
    pub fn new() -> Self {
        Self { state: GameState::new() }
    }

    pub fn apply(&mut self, action: GameAction) -> Option<GameEvent> {
        // TODO : add some sort of player validation before applying actions.
        // each client should hold a secret token assigned to each seat at the 
        // beginning of the game.
        match action {
            GameAction::Pass | GameAction::OrderUp | _ => {
                self.state.submit_bid(BidAction::Pass);
                Some(GameEvent::PhaseChanged(self.state.current_phase))
            }
            GameAction::PlayCard(i) => {
                self.state.play_card(self.state.current_player, i);
                self.state.handle_trick();

                match self.state.current_phase {
                    Phase::Scoring => {
                        self.state.score_round();
                        if self.state.current_phase == Phase::GameOver {
                            Some(GameEvent::GameOver { winner: Team::East})
                        } else {
                            self.state.new_deal();
                            self.state.start_bidding();
                            Some(GameEvent::NewHand)
                        }
                    }
                    _ => Some(GameEvent::PhaseChanged(self.state.current_phase))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card::{Card, Rank, Suit};
    use crate::game::player::Team;
    use crate::game::state::Phase;

    fn setup_known_hand() -> GameController {
        let mut controller = GameController::new();
        let state = &mut controller.state;

        state.players[0].hand.extend(vec![
            Card::new(Suit::Clubs, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Jack), 
            Card::new(Suit::Hearts, Rank::Nine),
            Card::new(Suit::Clubs, Rank::Jack),
        ]);
        state.players[1].hand.extend(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Nine),
            Card::new(Suit::Diamonds, Rank::Ace), 
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Spades, Rank::Ten),
        ]);
        state.players[2].hand.extend(vec![
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
            Card::new(Suit::Clubs, Rank::King), 
            Card::new(Suit::Diamonds, Rank::Jack),
            Card::new(Suit::Hearts, Rank::King),
        ]);
        state.players[3].hand.extend(vec![
            Card::new(Suit::Diamonds, Rank::Nine),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Ten), 
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Ten),
        ]);

        state.trump = Some(Suit::Spades);
        state.maker_team = Some(Team::East);
        state.current_phase = Phase::Playing;
        state.current_player = 1;

        controller
    }

    #[test]
    fn test_play_card_emits_trick_complete() {
        let mut controller = setup_known_hand();

        controller.apply(GameAction::PlayCard(3));
        controller.apply(GameAction::PlayCard(2));
        controller.apply(GameAction::PlayCard(4));
        let event = controller.apply(GameAction::PlayCard(0));

        assert!(matches!(event, Some(GameEvent::TrickComplete { winner: 0})));
        assert_eq!(controller.state.tricks_won, [1, 0]);
    }

    fn test_full_hand_scores_correctly() {
        let mut controller = setup_known_hand();

        controller.apply(GameAction::PlayCard(3));
        controller.apply(GameAction::PlayCard(2));
        controller.apply(GameAction::PlayCard(4));
        controller.apply(GameAction::PlayCard(0));

        controller.apply(GameAction::PlayCard(1));
        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(3));
        controller.apply(GameAction::PlayCard(2));

        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(0));

        controller.apply(GameAction::PlayCard(1));
        controller.apply(GameAction::PlayCard(1));
        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(0));

        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(0));
        controller.apply(GameAction::PlayCard(0));

        assert_eq!(controller.state.team_scores, [1, 0]);
        assert_eq!(controller.state.current_phase, Phase::Bidding);
    }

    // fn test_game_over_event() {
    //     let mut controller = GameController::new();
    //     controller.state.team_scores = [9, 0];
    //     let event = controller.apply(GameAction::PlayCard(0));
    //     assert_eq!(matches!(event, Some(GameEvent::GameOver { winner: Team::East })));
    // }
}
