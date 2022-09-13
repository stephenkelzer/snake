#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub row: usize,    // y
    pub column: usize, // x
}

impl Position {
    pub fn xy(x: usize, y: usize) -> Self {
        Self { row: y, column: x }
    }
}
