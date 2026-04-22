use euchre::game::card::{Card, Suit};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

// Portrait card outer dimensions (including border).
// Pixel ratio: CARD_W : CARD_H*2 = 8:10 = 4:5, close to a real card (5:7).
pub const CARD_W: u16 = 8;
pub const CARD_H: u16 = 5;

// Landscape card outer dimensions — portrait rotated 90°.
// Derived so the pixel ratio is identical: CARD_W_LAND : CARD_H_LAND*2 = 10:8 = 5:4.
pub const CARD_W_LAND: u16 = CARD_H * 2; // 10
pub const CARD_H_LAND: u16 = CARD_W / 2; // 4

fn render_suit(suit: Suit) -> String {
    match suit {
        Suit::Spades => "♠".to_string(),
        Suit::Clubs => "♣".to_string(),
        Suit::Hearts => "♥".to_string(),
        Suit::Diamonds => "♦".to_string(),
    }
}

fn render_card(card: &Card) -> Span<'static> {
    let suit_symbol = render_suit(card.suit);

    let display_string = format!("{}{}", card.rank, suit_symbol);
    let color = match card.suit {
        Suit::Spades | Suit::Clubs => Color::White,
        Suit::Hearts | Suit::Diamonds => Color::Red,
    };
    Span::styled(display_string, Style::default().fg(color))
}

pub fn render_hand(hand: &[Card], selected: Option<usize>, area: Rect, frame: &mut Frame) {
    let horizontal_chunks = Layout::horizontal(vec![Constraint::Length(CARD_W); hand.len()]).split(area);

    for (i, card) in hand.iter().enumerate() {
        let block = if selected == Some(i) {
            Block::bordered()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::Yellow))
                .border_type(BorderType::Thick)
        } else {
            Block::bordered()
        };

        let p = Paragraph::new(Line::from(render_card(card))).block(block);
        frame.render_widget(p, horizontal_chunks[i]);
    }
}

pub fn render_card_back(area: Rect, frame: &mut Frame) {
    frame.render_widget(Block::bordered(), area);
}

pub fn render_hand_back(count: usize, area: Rect, frame: &mut Frame) {
    let chunks = Layout::horizontal(vec![Constraint::Length(CARD_W); count]).split(area);
    for chunk in chunks.iter() {
        render_card_back(*chunk, frame);
    }
}

pub fn render_hand_back_vertical(count: usize, area: Rect, frame: &mut Frame) {
    let centered = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(CARD_W_LAND),
        Constraint::Fill(1),
    ])
    .split(area)[1];
    let chunks = Layout::vertical(vec![Constraint::Length(CARD_H_LAND); count]).split(centered);
    for chunk in chunks.iter() {
        render_card_back(*chunk, frame);
    }
}

pub fn render_score(scores: [usize; 2], area: Rect, trump: Option<Suit>, phase: &str, frame: &mut Frame) {
    let display_scores = format!("EAST: {} | WEST: {}", scores[0], scores[1]);
    let display_trump = match trump {
        Some(suit) => format!("Trump: {}", render_suit(suit)),
        None => "Trump: --".to_string(),
    };
    let display_phase = format!("Phase: {}", phase);

    let vertical_chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(area);

    frame.render_widget(Paragraph::new(display_scores), vertical_chunks[0]);
    frame.render_widget(Paragraph::new(display_trump), vertical_chunks[1]);
    frame.render_widget(Paragraph::new(display_phase), vertical_chunks[2]);
}

pub fn render_player_slot(_pos: usize, is_active: bool, label: &str, area: Rect, frame: &mut Frame) -> Rect {
    let block = if is_active {
        Block::bordered()
            .border_style(Style::new().fg(Color::Yellow))
            .title(label)
            .border_type(BorderType::Thick)
    } else {
        Block::bordered().title(label)
    };

    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}
