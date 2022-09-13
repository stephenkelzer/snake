use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub subtitle: String,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    html!(
        <div id={"game-status-wrapper"}>
            <div id={"game-status-title"}>{ props.title.to_string() }</div>
            <div id={"game-status-subtitle"}>{ props.subtitle.to_string() }</div>
        </div>
    )
}
