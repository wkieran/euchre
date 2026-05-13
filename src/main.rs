mod app;
mod tui;

use async_trait::async_trait;
use crossterm::{
    ExecutableCommand,
    event::{self, Event},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use euchre::game::{
    card::Suit,
    controller::{GameAction, GameController, GameView, PlayerInput},
    human::HumanPlayerInput,
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

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    event_rx: mpsc::Receiver<euchre::game::controller::GameEvent>,
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
