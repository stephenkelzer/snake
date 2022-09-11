use yew::{html, Component, Properties};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cell {
    SnakeHead,
    SnakeBody,
    Food,
    Empty,
}

#[derive(Debug, Properties, PartialEq)]
pub struct CellProps {
    cell: Cell,
}

impl Component for Cell {
    type Message = ();
    type Properties = CellProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.props().cell
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        let cell_content = match self {
            Cell::SnakeHead => "❇️",
            Cell::SnakeBody => "🟩",
            Cell::Food => "🍎",
            Cell::Empty => "",
        };

        html!(
            <div class={"cell"}>
                { cell_content }
            </div>
        )
    }
}
