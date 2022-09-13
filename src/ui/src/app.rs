use yew::{function_component, html, Properties};

use crate::components::game::Game;

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub version: String,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    html! {
        <div id={"app"} version={ props.version.clone() }>
            <Game />
        </div>
    }
}
