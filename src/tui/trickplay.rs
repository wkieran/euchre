// src/tui/trickplay.rs

use crate::{
    app::App,
    tui::components::{
        CARD_H_LAND, CARD_W, render_hand, render_hand_back, render_hand_back_vertical,
        render_player_slot, render_score,
    },
};
use euchre::game::card::{Card, Rank, Suit};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
};

pub fn render(frame: &mut Frame, _app: &App, area: Rect) {
    let outer = Layout::vertical([
        Constraint::Length(7),
        Constraint::Fill(1),
        Constraint::Length(7),
    ])
    .split(area);

    let middle = Layout::horizontal([
        Constraint::Length(20),
        Constraint::Fill(1),
        Constraint::Length(20),
    ])
    .split(outer[1]);

    let left_slot = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(CARD_H_LAND * 5 + 2),
        Constraint::Fill(1),
    ])
    .split(middle[0])[1];

    let right_slot = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(CARD_H_LAND * 5 + 2),
        Constraint::Fill(1),
    ])
    .split(middle[2])[1];

    // Score (3) + cross play area (15) grouped and centered in center column
    let center_group_area = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(18),
        Constraint::Fill(1),
    ])
    .split(middle[1])[1];

    let center_group = Layout::vertical([Constraint::Length(3), Constraint::Length(15)])
        .split(center_group_area);

    let score_area = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(22),
        Constraint::Fill(1),
    ])
    .split(center_group[0])[1];

    // Cross: N on top row, W+E in middle row, S on bottom row
    let cross_rows = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ])
    .split(center_group[1]);

    let n_slot = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(CARD_W),
        Constraint::Fill(1),
    ])
    .split(cross_rows[0])[1];

    let we_cols = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(CARD_W),
        Constraint::Length(5),
        Constraint::Length(CARD_W),
        Constraint::Fill(1),
    ])
    .split(cross_rows[1]);
    let w_slot = we_cols[1];
    let e_slot = we_cols[3];

    let s_slot = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(CARD_W),
        Constraint::Fill(1),
    ])
    .split(cross_rows[2])[1];

    let hand_area = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(CARD_W * 5 + 2),
        Constraint::Fill(1),
    ])
    .split(outer[2])[1];

    let test_hand = vec![
        Card::new(Suit::Spades, Rank::Ace),
        Card::new(Suit::Hearts, Rank::Jack),
        Card::new(Suit::Clubs, Rank::King),
        Card::new(Suit::Diamonds, Rank::Nine),
        Card::new(Suit::Spades, Rank::Ten),
    ];

    let top_slot = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(CARD_W * 5 + 2),
        Constraint::Fill(1),
    ])
    .split(outer[0])[1];
    let top_inner = render_player_slot(0, false, "test-label-0", top_slot, frame);
    render_hand_back(5, top_inner, frame);

    let left_inner = render_player_slot(1, true, "test-label-1", left_slot, frame);
    render_hand_back_vertical(5, left_inner, frame);

    let right_inner = render_player_slot(2, false, "test-label-2", right_slot, frame);
    render_hand_back_vertical(5, right_inner, frame);

    let hand_block = Block::bordered().title("Your Hand");
    let inner_hand_area = hand_block.inner(hand_area);
    frame.render_widget(hand_block, hand_area);
    render_hand(&test_hand, Some(0), inner_hand_area, frame);

    render_score([1, 0], score_area, Some(Suit::Spades), "Playing", frame);

    frame.render_widget(Block::bordered(), n_slot);
    frame.render_widget(Block::bordered(), w_slot);
    frame.render_widget(Block::bordered(), e_slot);
    frame.render_widget(Block::bordered(), s_slot);
}
