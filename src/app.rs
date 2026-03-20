// src/app.rs

use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, PartialEq, Clone)]
pub enum Page {
    Home,
    Dealing,
    Bidding,
    TrickPlay,
    Scoring,
    GameOver,
}

pub struct App {
    pub page: Page,
    pub tick_count: u64,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            page: Page::Home,
            tick_count: 0,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('1') => self.page = Page::Home,
            KeyCode::Char('2') => self.page = Page::Dealing,
            KeyCode::Char('3') => self.page = Page::Bidding,
            KeyCode::Char('4') => self.page = Page::TrickPlay,
            KeyCode::Char('5') => self.page = Page::Scoring,
            KeyCode::Char('6') => self.page = Page::GameOver,
            _ => {}
        }
    }
}


