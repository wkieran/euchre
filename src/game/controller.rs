// src/game/controller.rs
use super::card::{Card, Suit};
use super::player::Team;
use super::state::{BidAction, GameState, Phase};
use super::trick::Trick;
use async_trait::async_trait;

pub enum GameAction {
    Bid(BidAction),
    Discard(usize),
    PlayCard(usize),
}

#[derive(Clone)]
pub enum GameEvent {
    NewHand,
    BiddingStarted {
        kitty: Card,
    },
    PhaseChanged(Phase),
    TrickComplete {
        winner: usize,
        cards_played: Trick,
    },
    HandComplete {
        scores: [usize; 2],
        tricks_won: [usize; 2],
        maker_team: Team,
        points_awarded: usize,
    },
    GameOver {
        winner: Team,
    },
}

pub struct GameView {
    pub seat_id: usize,
    pub hand: Vec<Card>,
    pub current_phase: Phase,
    pub current_player: usize,
    pub current_bidder: usize,
    pub bidding_round: usize,
    pub current_trick: Trick,
    pub kitty: Option<Card>,
    pub trump: Option<Suit>,
    pub tricks_won: [usize; 2],
    pub team_scores: [usize; 2],
    pub dealer: usize,
}

#[async_trait]
pub trait PlayerInput: Send {
    async fn act(&mut self, view: &GameView) -> GameAction;
    async fn on_event(&mut self, _event: &GameEvent) {}
}

pub struct GameController {
    pub state: GameState,
    players: [Box<dyn PlayerInput>; 4],
}

impl GameController {
    pub fn new(players: [Box<dyn PlayerInput>; 4]) -> Self {
        Self {
            state: GameState::new(),
            players,
        }
    }

    pub async fn run(&mut self) -> Team {
        self.state.new_deal();
        self.state.start_bidding();
        self.broadcast(GameEvent::BiddingStarted {
            kitty: self.state.kitty.unwrap(),
        })
        .await;

        loop {
            let events = self.step().await;
            for event in &events {
                self.broadcast(event.clone()).await;
                if let GameEvent::GameOver { winner } = event {
                    return *winner;
                }
            }
        }
    }

    async fn step(&mut self) -> Vec<GameEvent> {
        let seat = match self.state.current_phase {
            Phase::Bidding | Phase::StuckDealer => self.state.current_bidder,
            Phase::Discarding => self.state.dealer,
            Phase::Playing => self.state.current_player,
            Phase::Scoring | Phase::Dealing | Phase::GameOver => return vec![],
        };
        let view = self.view_for(seat);
        let action = self.players[seat].act(&view).await;
        self.apply(action)
    }

    pub fn apply(&mut self, action: GameAction) -> Vec<GameEvent> {
        match action {
            GameAction::Bid(bid) => {
                self.state.submit_bid(bid);
                vec![GameEvent::PhaseChanged(self.state.current_phase)]
            }
            GameAction::Discard(card_index) => {
                self.state.submit_discard(card_index);
                vec![GameEvent::PhaseChanged(self.state.current_phase)]
            }
            GameAction::PlayCard(card_index) => {
                let tricks_before = self.state.tricks_won[0] + self.state.tricks_won[1];
                let player_id = self.state.current_player;
                self.state.play_card(player_id, card_index);
                let trick_snapshot = self.state.current_trick.clone();
                self.state.handle_trick();
                let tricks_after = self.state.tricks_won[0] + self.state.tricks_won[1];

                if self.state.current_phase == Phase::Scoring {
                    let tricks_won = self.state.tricks_won;
                    let maker_team = self.state.maker_team.unwrap();
                    let scores_before = self.state.team_scores;
                    self.state.score_round();
                    let scores_after = self.state.team_scores;
                    let points_awarded = (scores_after[0] + scores_after[1])
                        - (scores_before[0] + scores_before[1]);

                    let hand_complete = GameEvent::HandComplete {
                        scores: scores_after,
                        tricks_won,
                        maker_team,
                        points_awarded,
                    };

                    if self.state.current_phase == Phase::GameOver {
                        let winner = if self.state.team_scores[0] >= 10 {
                            Team::East
                        } else {
                            Team::West
                        };
                        vec![hand_complete, GameEvent::GameOver { winner }]
                    } else {
                        self.state.new_deal();
                        self.state.start_bidding();
                        vec![hand_complete, GameEvent::NewHand]
                    }
                } else if tricks_after > tricks_before {
                    vec![GameEvent::TrickComplete {
                        winner: self.state.current_player,
                        cards_played: trick_snapshot,
                    }]
                } else {
                    vec![GameEvent::PhaseChanged(self.state.current_phase)]
                }
            }
        }
    }

    pub fn view_for(&self, seat: usize) -> GameView {
        GameView {
            seat_id: seat,
            hand: self.state.players[seat].hand.clone(),
            current_phase: self.state.current_phase,
            current_player: self.state.current_player,
            current_bidder: self.state.current_bidder,
            bidding_round: self.state.bidding_round,
            current_trick: self.state.current_trick.clone(),
            kitty: self.state.kitty,
            trump: self.state.trump,
            tricks_won: self.state.tricks_won,
            team_scores: self.state.team_scores,
            dealer: self.state.dealer,
        }
    }

    async fn broadcast(&mut self, event: GameEvent) {
        for p in &mut self.players {
            p.on_event(&event).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card::{Card, Rank, Suit};
    use crate::game::player::Team;
    use crate::game::state::Phase;

    struct StubPlayer;

    #[async_trait]
    impl PlayerInput for StubPlayer {
        async fn act(&mut self, _view: &GameView) -> GameAction {
            unreachable!("StubPlayer should not be asked to act in unit tests")
        }
    }

    fn stub_players() -> [Box<dyn PlayerInput>; 4] {
        [
            Box::new(StubPlayer),
            Box::new(StubPlayer),
            Box::new(StubPlayer),
            Box::new(StubPlayer),
        ]
    }

    fn setup_known_hand() -> GameController {
        let mut controller = GameController::new(stub_players());
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
        let events = controller.apply(GameAction::PlayCard(0));

        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], GameEvent::TrickComplete { winner: 0, .. }));
        assert_eq!(controller.state.tricks_won, [1, 0]);
    }

    #[test]
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
