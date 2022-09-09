use crate::random::random_range;
use std::collections::VecDeque;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>, // head is first item, tail is last item inside of vector
    pub food: Position,
    direction: Direction,
    next_direction: Direction,
    finished: bool,
}

impl SnakeGame {
    pub fn new(size: usize) -> Self {
        Self {
            width: size,
            height: size,
            snake: [((size - 3).max(0), size / 2)].into_iter().collect(),
            direction: Direction::Left,
            next_direction: Direction::Left,
            food: (2.min(size - 1), size / 2),
            finished: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.finished {
            return;
        }

        match (&self.direction, direction) {
            (Direction::Up, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Down, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => {}
            (_, direction) => self.next_direction = direction,
        }
    }

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.finished && self.snake.len() == 0 {
            return;
        }

        self.direction = self.next_direction;

        let (x, y) = self.snake[0];
        let new_head = match &self.direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.finished = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_positions = (9..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    .filter(|pos| !self.snake.contains(pos))
                    .collect::<Vec<_>>();

                if free_positions.is_empty() {
                    self.finished = true;
                    return;
                }

                self.food = free_positions[random_range(0, free_positions.len())];
            }

            self.snake.push_front(new_head);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10));
    }
}
