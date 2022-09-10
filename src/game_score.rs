use wasm_react::{c, h, Component};

#[derive(Debug)]
pub struct GameScore {
    score: u32,
}

impl Component for GameScore {
    fn render(&self) -> wasm_react::VNode {
        h!(div)
            .id("game-score")
            .build(c![format!("Score: {}", self.score)])
    }
}

impl GameScore {
    pub fn new(score: u32) -> Self {
        Self { score }
    }
}
