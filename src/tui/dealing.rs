// src/tui/dealing.rs

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let dots = match app.tick_count % 4 {
        0 => "",
        1 => ".",
        2 => "..",
        _ => "...",
    };

    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .split(area);

    let center = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(20),
        Constraint::Fill(1),
    ])
    .split(vertical[1])[1];

    frame.render_widget(
        Paragraph::new(format!("Dealing{}", dots)).block(Block::bordered()),
        center,
    );
}
