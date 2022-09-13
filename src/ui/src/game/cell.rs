use yew::{function_component, html, Properties};

use game::element::Element;
use game::position::Position as LogicalPosition;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub position: LogicalPosition,
    pub element: Option<Element>,
}

#[function_component(Cell)]
pub fn cell(props: &Props) -> Html {
    let cell_content = match props.element {
        Some(Element::SnakeHead) => "❇️",
        Some(Element::SnakeBody) => "🟩",
        Some(Element::Food) => "🍎",
        None => "",
    };

    html! {
        <div class={"cell"} data-row={ props.position.row.to_string() } data-col={ props.position.column.to_string() }>
            { cell_content }
        </div>
    }
}
