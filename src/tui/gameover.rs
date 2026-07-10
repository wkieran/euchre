// src/tui/gameover.rs

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame, _app: &App, area: Rect) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(10),
        Constraint::Fill(1),
    ])
    .split(area);

    let center = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(35),
        Constraint::Fill(1),
    ])
    .split(vertical[1])[1];

    let content = "EAST WINS!\n\nFinal Scores\nEAST: 10 | WEST: 7\n\nPress q to quit";
    frame.render_widget(
        Paragraph::new(content).block(Block::bordered().title("Game Over")),
        center,
    );
}
