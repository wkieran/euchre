mod bidding;
mod components;
mod dealing;
mod gameover;
mod home;
mod scoring;
mod trickplay;

use crate::app::{App, Page};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    render_nav(frame, app, chunks[0]);

    match app.page {
        Page::Home => home::render(frame, app, chunks[1]),
        Page::Dealing => dealing::render(frame, app, chunks[1]),
        Page::Bidding => bidding::render(frame, app, chunks[1]),
        Page::TrickPlay => trickplay::render(frame, app, chunks[1]),
        Page::Scoring => scoring::render(frame, app, chunks[1]),
        Page::GameOver => gameover::render(frame, app, chunks[1]),
    }
}

fn render_nav(frame: &mut Frame, app: &App, area: Rect) {
    let nav = Paragraph::new(Line::from(vec![
        nav_span("1:Home", app.page == Page::Home),
        Span::raw(" "),
        nav_span("2:Dealing", app.page == Page::Dealing),
        Span::raw(" "),
        nav_span("3:Bidding", app.page == Page::Bidding),
        Span::raw(" "),
        nav_span("4:TrickPlay", app.page == Page::TrickPlay),
        Span::raw(" "),
        nav_span("5:Scoring", app.page == Page::Scoring),
        Span::raw(" "),
        nav_span("6:GameOver", app.page == Page::GameOver),
        Span::raw(" | q:Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Nav"));
    frame.render_widget(nav, area);
}

fn nav_span(label: &str, active: bool) -> Span<'_> {
    if active {
        Span::styled(
            label,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::raw(label)
    }
}
