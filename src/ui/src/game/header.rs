use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String,
    pub subtitle: String,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html!(
        <div id={"game-status-wrapper"}>
            <div id={"game-status-title"}>{ props.title.to_string() }</div>
            <div id={"game-status-subtitle"}>{ props.subtitle.to_string() }</div>
        </div>
    )
}
