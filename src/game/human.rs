use crate::game::controller::{GameAction, GameEvent, GameView, PlayerInput};
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct HumanPlayerInput {
    action_rx: mpsc::Receiver<GameAction>,
    event_tx: mpsc::Sender<(GameEvent, GameView)>,
}

impl HumanPlayerInput {
    pub fn new() -> (
        Self,
        mpsc::Sender<GameAction>,
        mpsc::Receiver<(GameEvent, GameView)>,
    ) {
        let (action_tx, action_rx) = mpsc::channel(32);
        let (event_tx, event_rx) = mpsc::channel(32);
        (
            Self {
                action_rx,
                event_tx,
            },
            action_tx,
            event_rx,
        )
    }
}

#[async_trait]
impl PlayerInput for HumanPlayerInput {
    async fn act(&mut self, _view: &GameView) -> GameAction {
        self.action_rx.recv().await.unwrap()
    }
    async fn on_event(&mut self, event: &GameEvent, view: &GameView) {
        let _ = self.event_tx.send((event.clone(), view.clone())).await;
    }
}
