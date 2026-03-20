// src/tui/home.rs

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};
use crate::app::App;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let content = format!(
        "Welcome home!\n\nTick count: {}\n\n(Ticks increment every 250ms via the poll loop)",
        app.tick_count
    );
    let p = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Home"));
    frame.render_widget(p, area);
}
