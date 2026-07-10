// src/tui/scoring.rs

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame, _app: &App, area: Rect) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(12),
        Constraint::Fill(1),
    ])
    .split(area);

    let center = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(35),
        Constraint::Fill(1),
    ])
    .split(vertical[1])[1];

    let content = "EAST wins the hand!\n\nTricks: EAST 3 | WEST 2\nPoints: +1\n\nEAST: 1 | WEST: 0\n\nPress Enter to continue";
    frame.render_widget(
        Paragraph::new(content).block(Block::bordered().title("Scoring")),
        center,
    );
}
