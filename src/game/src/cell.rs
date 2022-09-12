#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cell {
    SnakeHead,
    SnakeBody,
    Food,
    Empty,
}
