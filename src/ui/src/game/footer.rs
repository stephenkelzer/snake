use yew::{html, Properties};

use yew::function_component;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub score: u32,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    html!(
        <div id={"game-score"}>
            { format!("Score: {}", props.score) }
        </div>
    )
}
