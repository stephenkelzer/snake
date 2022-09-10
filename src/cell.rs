use wasm_react::{c, h, VNode};

#[derive(Debug)]
pub enum Cell {
    SnakeHead,
    SnakeBody,
    Food,
    Empty,
}

impl Cell {
    pub fn render(self) -> VNode {
        let cell_content = match self {
            Cell::SnakeHead => "❇️",
            Cell::SnakeBody => "🟩",
            Cell::Food => "🍎",
            Cell::Empty => "",
        };

        return h!(div).class_name("cell").build(c![cell_content]);
    }
}
