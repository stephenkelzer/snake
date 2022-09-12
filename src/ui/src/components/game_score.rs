use yew::{html, Properties};

use yew::function_component;

#[derive(Properties, PartialEq)]
pub struct GameScoreProps {
    pub score: u32,
}

#[function_component(GameScore)]
pub fn gameScore(props: &GameScoreProps) -> Html {
    html!(
        <div id={"game-score"}>
            { format!("Score: {}", props.score) }
        </div>
    )
}
