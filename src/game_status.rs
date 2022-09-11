use yew::{html, Component};

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Playing,
    Paused,
    GameOver,
}

impl Component for GameStatus {
    type Message = ();
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        GameStatus::Paused
    }

    // lets leave this commented out to see what happens
    // fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    //     false
    // }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let (title, subtitle) = match self {
            GameStatus::Playing => (
                "Playing",
                "Use the arrow keys (or 'WASD') to move the snake.",
            ),
            GameStatus::Paused => (
                "Paused",
                "Click on the game area or press 'space' to resume",
            ),
            GameStatus::GameOver => ("Game Over!", "Refresh your browser to play again."),
        };

        html!(
            <div id={"game-status-wrapper"}>
                <div id={"game-status-title"}>{ title }</div>
                <div id={"game-status-subtitle"}>{ subtitle }</div>
            </div>
        )
    }
}
