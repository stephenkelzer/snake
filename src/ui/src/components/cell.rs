use yew::{function_component, html, Properties};

use game::cell::Cell as LogicalCell;
use game::position::Position as LogicalPosition;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub position: LogicalPosition,
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
        <div class={"cell"} data-row={ props.position.row.to_string() } data-col={ props.position.column.to_string() }>
            { cell_content }
        </div>
    }
}
