use rand::Rng;

use crate::{
    collidable::Collidable, direction::Direction, food::Food, position::Position, snake::Snake,
};

#[derive(Debug)]
pub struct Game {
    pub size: usize,
    pub snake: Snake,
    pub food: Food,
    pub finished: bool,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            snake: Snake::new(((size - 3).max(0), size / 2), Direction::Left),
            food: Food::new((2.min(size - 1), size / 2)),
            finished: false,
        }
    }

    pub fn handle_key_press(&mut self, key_code: String) {
        if self.finished {
            return;
        }

        self.snake.handle_key_press(key_code)
    }

    pub fn handle_tick(&mut self) {
        if self.finished {
            return;
        }

        let new_position = self.snake.get_next_position();

        if self.snake.check_for_collision(&new_position) || self.is_out_of_bounds(new_position) {
            self.finished = true;
            return;
        }

        let collides_with_food = self.food.check_for_collision(&new_position);

        self.snake.add_new_head(new_position, !collides_with_food);

        if collides_with_food {
            self.food = Food::new(self.get_random_available_position());
        }
    }

    fn is_out_of_bounds(&self, position: Position) -> bool {
        let (x, y) = position;
        x > self.size || y > self.size
    }

    fn get_available_positions(&self) -> Vec<Position> {
        (0..self.size)
            .flat_map(|y| (0..self.size).map(move |x| (x, y)))
            .filter(|pos| {
                !self.snake.check_for_collision(&pos) && !self.food.check_for_collision(&pos)
            })
            .collect::<Vec<Position>>()
    }

    fn get_random_available_position(&self) -> Position {
        // let available_positions = self.get_available_positions();
        // available_positions[random_range(0, available_positions.len())]

        let available_positions = self.get_available_positions();
        available_positions[rand::thread_rng().gen_range(0..(available_positions.len() + 1))]
    }
}
