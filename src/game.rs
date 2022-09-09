use crate::{direction::Direction, position::Position, random::random_range, snake::Snake};

#[derive(Debug)]
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub snake: Snake,
    pub food: Position,
    pub finished: bool,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            width: size,
            height: size,
            snake: Snake::new(((size - 3).max(0), size / 2), Direction::Left),
            food: (2.min(size - 1), size / 2),
            finished: false,
        }
    }

    pub fn handle_key_press(&mut self, key_code: String) {
        if self.finished {
            return;
        }

        self.snake.handle_key_press(key_code)
    }

    pub fn tick(&mut self) {
        if self.finished {
            return;
        }

        let new_position = self.snake.get_next_position().unwrap();

        if !self.is_valid(new_position) || self.snake.positions.contains(&new_position) {
            self.finished = true;
            return;
        }

        let ate_food = new_position == self.food;

        self.snake.add_new_head(new_position, !ate_food);

        if ate_food {
            let available_positions = self.get_available_positions();
            self.food = available_positions[random_range(0, available_positions.len())];
        }
    }

    fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    fn get_available_positions(&self) -> Vec<Position> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|pos| !self.snake.positions.contains(pos) && self.food != *pos)
            .collect::<Vec<Position>>()
    }
}
