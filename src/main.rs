mod app;
mod tui;

use async_trait::async_trait;
use crossterm::{
    ExecutableCommand,
    event::{self, Event},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use euchre::game::{
    card::{Suit, effective_suit},
    controller::{GameAction, GameController, GameEvent, GameView, PlayerInput},
    human::HumanPlayerInput,
    player::Team,
    state::{BidAction, Phase},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;

use app::App;

struct PassBot;

#[async_trait]
impl PlayerInput for PassBot {
    async fn act(&mut self, view: &GameView) -> GameAction {
        match view.current_phase {
            Phase::Bidding => GameAction::Bid(BidAction::Pass),
            Phase::StuckDealer => GameAction::Bid(BidAction::CallSuit(Suit::Spades)),
            Phase::Discarding => GameAction::Discard(0),
            Phase::Playing => GameAction::PlayCard(0),
            _ => unreachable!(),
        }
    }
}

struct DebugBot {
    seat: usize,
}

#[async_trait]
impl PlayerInput for DebugBot {
    async fn act(&mut self, view: &GameView) -> GameAction {
        let action = match view.current_phase {
            Phase::Bidding => GameAction::Bid(BidAction::Pass),
            Phase::StuckDealer => {
                let kitty_suit = view.kitty.unwrap().suit;
                let call_suit = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]
                    .into_iter()
                    .find(|&s| s != kitty_suit)
                    .unwrap();
                GameAction::Bid(BidAction::CallSuit(call_suit))
            }
            Phase::Discarding => GameAction::Discard(0),
            Phase::Playing => {
                let trump = view.trump.unwrap();
                let idx = if let Some(lead_suit) = view.current_trick.lead_suit {
                    view.hand
                        .iter()
                        .position(|c| effective_suit(*c, trump) == lead_suit)
                        .unwrap_or(0)
                } else {
                    0
                };
                GameAction::PlayCard(idx)
            }
            _ => unreachable!(),
        };
        if self.seat == 0 {
            let label = match &action {
                GameAction::Bid(BidAction::Pass) => "Pass".to_string(),
                GameAction::Bid(BidAction::OrderUp) => "OrderUp".to_string(),
                GameAction::Bid(BidAction::CallSuit(s)) => format!("CallSuit({s})"),
                GameAction::Discard(i) => format!("Discard({i})"),
                GameAction::PlayCard(i) => format!("PlayCard({i}) → {}", view.hand[*i]),
                _ => "?".to_string(),
            };
            println!("[seat {}] act: {label}", self.seat);
        }
        action
    }

    async fn on_event(&mut self, event: &GameEvent, _view: &GameView) {
        if self.seat != 0 {
            return;
        }
        match event {
            GameEvent::NewHand => println!("\n--- New Hand ---"),
            GameEvent::BiddingStarted { kitty } => println!("Bidding started | kitty: {kitty}"),
            GameEvent::PhaseChanged(phase) => println!("Phase → {phase:?}"),
            GameEvent::TrickComplete {
                winner,
                cards_played,
            } => {
                let cards: Vec<String> = cards_played
                    .played_cards
                    .iter()
                    .map(|(seat, card)| format!("{seat}:{card}"))
                    .collect();
                println!("Trick → seat {winner} wins  [{}]", cards.join("  "));
            }
            GameEvent::HandComplete {
                scores,
                tricks_won,
                maker_team,
                points_awarded,
            } => {
                let maker = match maker_team {
                    Team::East => "East",
                    Team::West => "West",
                };
                println!(
                    "Hand complete | tricks E:{} W:{} | maker:{maker} +{points_awarded}pt | scores E:{} W:{}",
                    tricks_won[0], tricks_won[1], scores[0], scores[1]
                );
            }
            GameEvent::GameOver { winner } => {
                let w = match winner {
                    Team::East => "East",
                    Team::West => "West",
                };
                println!("\n=== Game Over — {w} wins ===");
            }
        }
    }
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    event_rx: mpsc::Receiver<(GameEvent, GameView)>,
    action_tx: mpsc::Sender<GameAction>,
) -> io::Result<()> {
    let mut app = App::new(event_rx, action_tx);
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|frame| tui::render(frame, &app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                app.on_key(key);
            }
        } else {
            app.on_tick();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    if std::env::args().any(|a| a == "--debug") {
        let players: [Box<dyn PlayerInput>; 4] = [
            Box::new(DebugBot { seat: 0 }),
            Box::new(DebugBot { seat: 1 }),
            Box::new(DebugBot { seat: 2 }),
            Box::new(DebugBot { seat: 3 }),
        ];
        let mut controller = GameController::new(players);
        controller.run().await;
        return Ok(());
    }

    let (human, action_tx, event_rx) = HumanPlayerInput::new();

    let players: [Box<dyn PlayerInput>; 4] = [
        Box::new(human),
        Box::new(PassBot),
        Box::new(PassBot),
        Box::new(PassBot),
    ];

    let mut controller = GameController::new(players);
    tokio::spawn(async move { controller.run().await });

    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, event_rx, action_tx);

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_debug_game_runs_to_completion() {
        let players: [Box<dyn PlayerInput>; 4] = [
            Box::new(DebugBot { seat: 0 }),
            Box::new(DebugBot { seat: 1 }),
            Box::new(DebugBot { seat: 2 }),
            Box::new(DebugBot { seat: 3 }),
        ];
        let mut controller = GameController::new(players);
        controller.run().await;
        let scores = controller.state.team_scores;
        assert!(scores[0] >= 10 || scores[1] >= 10);
    }
}
