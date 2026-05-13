// src/app.rs

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use euchre::game::controller::{GameAction, GameEvent, GameView};
use tokio::sync::mpsc;

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
    pub home_selection: usize,
    pub room_code: String,
    pub event_rx: mpsc::Receiver<GameEvent>,
    pub action_tx: mpsc::Sender<GameAction>,
    pub view: Option<GameView>,
}

impl App {
    pub fn new(event_rx: mpsc::Receiver<GameEvent>, action_tx: mpsc::Sender<GameAction>) -> Self {
        Self {
            page: Page::Home,
            tick_count: 0,
            should_quit: false,
            home_selection: 0,
            room_code: String::new(),
            event_rx,
            action_tx,
            view: None,
        }
    }

    pub fn on_tick(&mut self) {
        self.tick_count += 1;
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                GameEvent::NewHand => self.page = Page::Dealing,
                GameEvent::BiddingStarted { kitty: _ } => self.page = Page::Bidding,
                GameEvent::PhaseChanged(_) => {}
                GameEvent::TrickComplete { winner: _ } => {}
                GameEvent::HandComplete { scores: _ } => self.page = Page::Scoring,
                GameEvent::GameOver { winner: _ } => self.page = Page::GameOver,
            }
        }
    }

    fn home_menu_up(&mut self) {
        self.home_selection = (self.home_selection + 2) % 3;
    }

    fn home_menu_down(&mut self) {
        self.home_selection = (self.home_selection + 1) % 3
    }

    fn home_handler(&mut self) {}

    fn on_key_home(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.home_menu_up(),
            KeyCode::Down => self.home_menu_down(),
            KeyCode::Char(c) => {
                if self.home_selection == 1 {
                    self.room_code.push(c);
                } else if c == 'q' {
                    self.should_quit = true;
                }
            }
            KeyCode::Backspace => {
                if self.home_selection == 1 {
                    self.room_code.pop();
                }
            }
            KeyCode::Enter => self.home_handler(),
            _ => {}
        }
    }

    fn on_key_global(&mut self, key: KeyEvent) {
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

    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.page {
            Page::Home => {
                self.on_key_home(key);
                self.on_key_global(key);
            }
            _ => self.on_key_global(key),
        }
    }
}
