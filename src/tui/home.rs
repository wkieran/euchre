// src/tui/home.rs

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, List, ListState, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let vertical_chunks = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(10),
        Constraint::Fill(1),
    ])
    .split(area);

    let horizontal_chunks = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(30),
        Constraint::Fill(1),
    ])
    .split(vertical_chunks[1]);

    let inner_chunks = Layout::vertical([Constraint::Length(5), Constraint::Length(3)])
        .split(horizontal_chunks[1]);

    let mut state = ListState::default();
    state.select(Some(app.home_selection));
    let items = ["Start new game", "Join game", "Quit"];
    let list = List::new(items)
        .block(Block::bordered().title("euchre"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    let p =
        Paragraph::new(format!("{}|", app.room_code)).block(Block::bordered().title("Room Code"));

    if app.home_selection == 1 {
        frame.render_widget(p, inner_chunks[1]);
    }

    frame.render_stateful_widget(list, inner_chunks[0], &mut state);
}
