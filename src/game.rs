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

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.finished && self.snake.positions.len() == 0 {
            return;
        }

        self.snake.handle_tick();

        let (x, y) = self.snake.positions[0];
        let new_head = match &self.snake.heading {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if !self.is_valid(new_head) || self.snake.positions.contains(&new_head) {
            self.finished = true;
        } else {
            if new_head != self.food {
                self.snake.positions.pop_back();
            } else {
                let free_positions = self.get_free_positions();

                if free_positions.is_empty() {
                    self.finished = true;
                    return;
                }

                self.food = free_positions[random_range(0, free_positions.len())];
            }

            self.snake.positions.push_front(new_head);
        }
    }

    fn get_free_positions(&self) -> Vec<Position> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|pos| !self.snake.positions.contains(pos))
            .collect::<Vec<Position>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn test() {
        println!("{:?}", Game::new(10));
    }
}
