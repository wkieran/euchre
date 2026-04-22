// src/tui/bidding.rs

use crate::{
    app::App,
    tui::components::{
        render_hand, render_hand_back, render_hand_back_vertical, render_player_slot, render_score,
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
        Constraint::Length(27),
        Constraint::Fill(1),
    ])
    .split(middle[0])[1];

    let right_slot = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(27),
        Constraint::Fill(1),
    ])
    .split(middle[2])[1];

    // Score + phase directly above kitty, grouped and centered in center column
    let center_group_area = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(10),
        Constraint::Fill(1),
    ])
    .split(middle[1])[1];

    let center_group = Layout::vertical([Constraint::Length(3), Constraint::Length(7)])
        .split(center_group_area);

    let score_area = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(22),
        Constraint::Fill(1),
    ])
    .split(center_group[0])[1];

    let kitty_area = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(9),
        Constraint::Fill(1),
    ])
    .split(center_group[1])[1];

    let hand_area = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(35),
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
        Constraint::Length(37),
        Constraint::Fill(1),
    ])
    .split(outer[0])[1];
    let top_inner = render_player_slot(0, false, "test-label-0", top_slot, frame);
    render_hand_back(5, top_inner, frame);

    let left_inner = render_player_slot(1, true, "test-label-1", left_slot, frame);
    let left_cards = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(7),
        Constraint::Fill(1),
    ])
    .split(left_inner)[1];
    render_hand_back_vertical(5, left_cards, frame);

    let right_inner = render_player_slot(2, false, "test-label-2", right_slot, frame);
    let right_cards = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(7),
        Constraint::Fill(1),
    ])
    .split(right_inner)[1];
    render_hand_back_vertical(5, right_cards, frame);

    let hand_block = Block::bordered().title("Your Hand");
    let inner_hand_area = hand_block.inner(hand_area);
    frame.render_widget(hand_block, hand_area);
    render_hand(&test_hand, Some(2), inner_hand_area, frame);

    render_score([1, 0], score_area, Some(Suit::Spades), "Bidding", frame);
    frame.render_widget(Block::bordered().title("Kitty"), kitty_area);
}
