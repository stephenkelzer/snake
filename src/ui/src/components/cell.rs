use yew::{function_component, html, Properties};

use game::cell::Cell as LogicalCell;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell: LogicalCell,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let cell_content = match props.cell {
        LogicalCell::SnakeHead => "❇️",
        LogicalCell::SnakeBody => "🟩",
        LogicalCell::Food => "🍎",
        LogicalCell::Empty => "",
    };

    html! {
        <div class={"cell"}>
            { cell_content }
        </div>
    }
}
