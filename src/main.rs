//src/main.rs
mod tui;
mod app;
// mod game;
// mod net;

// use game::{Card, Suit, Rank, Trick, Player, Team, Deck};

use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
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

fn main() -> io::Result<()> {
    enable_raw_mode();
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode();
    io::stdout().execute(LeaveAlternateScreen)?;
    
    result
}
