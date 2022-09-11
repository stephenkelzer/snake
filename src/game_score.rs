use yew::{html, Component, Properties};

#[derive(Debug, Properties, PartialEq)]
pub struct GameScore {
    score: u32,
}

impl Component for GameScore {
    type Message = ();

    type Properties = GameScore;

    fn create(ctx: &yew::Context<Self>) -> Self {
        GameScore::new(ctx.props().score)
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!(
            <div id={"game-score"}>
                { format!("Score: {}", self.score) }
            </div>
        )
    }
}

impl GameScore {
    pub fn new(score: u32) -> Self {
        Self { score }
    }
}
