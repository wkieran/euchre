use euchre::game::card::{Card, Suit};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

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
    let horizontal_chunks = Layout::horizontal(vec![Constraint::Length(7); hand.len()]).split(area);

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

pub fn render_hand_back(count: usize, area: Rect, frame: &mut Frame) {
    let chunks = Layout::horizontal(vec![Constraint::Length(7); count]).split(area);
    for i in 0..count {
        frame.render_widget(Block::bordered(), chunks[i]);
    }
}

pub fn render_hand_back_vertical(count: usize, area: Rect, frame: &mut Frame) {
    let chunks = Layout::vertical(vec![Constraint::Length(5); count]).split(area);
    for i in 0..count {
        frame.render_widget(Block::bordered(), chunks[i]);
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
