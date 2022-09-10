use wasm_react::{c, h, Component, VNode};

struct SnakeHeadCell;

impl Component for SnakeHeadCell {
    fn render(&self) -> wasm_react::VNode {
        h!(div).class_name("snake-head").build(c!["SH".to_string()])
    }
}

struct SnakeBodyCell;

impl Component for SnakeBodyCell {
    fn render(&self) -> wasm_react::VNode {
        h!(div).class_name("snake-body").build(c!["SB".to_string()])
    }
}

struct FoodCell;

impl Component for FoodCell {
    fn render(&self) -> wasm_react::VNode {
        h!(div).class_name("food").build(c!["F".to_string()])
    }
}

struct EmptyCell;

impl Component for EmptyCell {
    fn render(&self) -> wasm_react::VNode {
        h!(div).class_name("empty").build(c!["".to_string()])
    }
}

#[derive(Debug)]
pub enum Cell {
    SnakeHead,
    SnakeBody,
    Food,
    Empty,
}

impl Cell {
    pub fn render(self) -> VNode {
        return match self {
            Cell::SnakeHead => SnakeHeadCell.build(),
            Cell::SnakeBody => SnakeBodyCell.build(),
            Cell::Food => FoodCell.build(),
            Cell::Empty => EmptyCell.build(),
        };
    }
}
