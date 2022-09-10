use wasm_react::{c, h, Component, VNode};

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Playing,
    Paused,
    GameOver,
}

impl Component for GameStatus {
    fn render(&self) -> VNode {
        let (title, subtitle) = match self {
            GameStatus::Playing => (
                "Playing",
                "Use the arrow keys (or 'WASD') to move the snake",
            ),
            GameStatus::Paused => ("Paused", "Click on the game area to resume"),
            GameStatus::GameOver => ("Game Over!", "Refresh your browser to play again."),
        };

        h!(div).id("game-status-wrapper").build(c![
            h!(div).id("game-status-title").build(c![title]),
            h!(div).id("game-status-subtitle").build(c![subtitle])
        ])
    }
}
