use rand::Rng;

use crate::{
    cell::Cell, collidable::Collidable, direction::Direction, food::Food, game_status::GameStatus,
    position::Position, snake::Snake,
};

#[derive(Debug, PartialEq)]
pub struct Game {
    status: GameStatus,
    size: usize,
    snake: Snake,
    food: Food,
}

impl Game {
    pub fn new() -> Self {
        let size = 10;

        Self {
            size,
            snake: Snake::new(Position::xy((size - 3).max(0), size / 2), Direction::Left),
            food: Food::new(Position::xy(2.min(size - 1), size / 2)),
            status: GameStatus::Paused,
        }
    }

    pub fn handle_key_press(&mut self, key_code: String) {
        if key_code == "Space" {
            match self.status {
                GameStatus::Playing => self.pause(),
                GameStatus::Paused => self.unpause(),
                _ => {}
            }
        }

        if self.status == GameStatus::Playing {
            self.snake.handle_key_press(key_code)
        }
    }

    pub fn handle_tick(&mut self) {
        if self.status != GameStatus::Playing {
            return;
        }

        let new_position = self.snake.get_next_position();

        if self.snake.check_for_collision(&new_position) || self.is_out_of_bounds(new_position) {
            self.status = GameStatus::GameOver;
            return;
        }

        let collides_with_food = self.food.check_for_collision(&new_position);

        self.snake.add_new_head(new_position, !collides_with_food);

        if collides_with_food {
            self.food = Food::new(self.get_random_available_position());
        }
    }

    pub fn pause(&mut self) {
        if self.status == GameStatus::GameOver {
            return;
        }
        self.status = GameStatus::Paused;
    }

    pub fn unpause(&mut self) {
        if self.status == GameStatus::GameOver {
            return;
        }
        self.status = GameStatus::Playing;
    }

    pub fn get_table_layout(&self) -> Vec<(usize, Vec<(Position, Cell)>)> {
        let mut rows: Vec<(usize, Vec<(Position, Cell)>)> = vec![];

        for row in 0..self.size {
            let mut columns: Vec<(Position, Cell)> = vec![];
            for column in 0..self.size {
                let position = Position { row, column };
                let cell = match position {
                    position if self.snake.is_head_position(position) => Cell::SnakeHead,
                    position if self.snake.check_for_collision(&position) => Cell::SnakeBody,
                    position if self.food.check_for_collision(&position) => Cell::Food,
                    _ => Cell::Empty,
                };
                columns.push((position, cell));
            }
            rows.push((row, columns));
        }

        return rows;
    }

    pub fn score(&self) -> u32 {
        self.snake.body_length()
    }

    pub fn is_game_over(&self) -> bool {
        self.status == GameStatus::GameOver
    }

    pub fn is_playing(&self) -> bool {
        self.status == GameStatus::Playing
    }

    pub fn is_paused(&self) -> bool {
        self.status == GameStatus::Paused
    }

    fn is_out_of_bounds(&self, position: Position) -> bool {
        position.column > (self.size - 1) || position.row > (self.size - 1)
    }

    fn get_random_available_position(&self) -> Position {
        let available_positions: Vec<Position> = self
            .get_table_layout()
            .into_iter()
            .flat_map(|(_, columns)| columns)
            .filter(|(_, cell)| cell == &Cell::Empty)
            .map(|(position, _)| position)
            .collect();

        available_positions[rand::thread_rng().gen_range(0..(available_positions.len() + 1))]
    }
}
