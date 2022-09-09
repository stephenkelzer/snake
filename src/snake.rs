use std::collections::VecDeque;

use crate::{direction::Direction, position::Position};

#[derive(Debug)]
pub struct Snake {
    pub positions: VecDeque<Position>, // head is first item, tail is last item inside of vector
    heading: Direction,
    next_heading: Direction,
}

impl Snake {
    pub fn new(starting_position: Position, starting_direction: Direction) -> Self {
        Self {
            heading: starting_direction,
            next_heading: starting_direction,
            positions: [starting_position].into_iter().collect(),
        }
    }

    pub fn handle_key_press(&mut self, key_code: String) {
        let requested_direction = match &key_code[..] {
            "ArrowUp" | "w" => Some(Direction::Up),
            "ArrowRight" | "d" => Some(Direction::Right),
            "ArrowDown" | "s" => Some(Direction::Down),
            "ArrowLeft" | "a" => Some(Direction::Left),
            _ => None,
        };

        if let Some(direction) = requested_direction {
            let next_direction = match (&self.heading, direction) {
                (Direction::Up, Direction::Up)
                | (Direction::Up, Direction::Down)
                | (Direction::Right, Direction::Right)
                | (Direction::Right, Direction::Left)
                | (Direction::Down, Direction::Up)
                | (Direction::Down, Direction::Down)
                | (Direction::Left, Direction::Right)
                | (Direction::Left, Direction::Left) => {
                    // ignore these options as they're invalid movements
                    None
                }
                (_, direction) => Some(direction),
            };

            if let Some(next_direction) = next_direction {
                self.next_heading = next_direction
            }
        }
    }

    pub fn get_next_position(&mut self) -> Result<Position, String> {
        self.heading = self.next_heading;

        let current_head = self.positions.front();
        if current_head.is_none() {
            return Err("Snake has no head!".into());
        }

        let (x, y) = *current_head.unwrap();

        let new_head: Position = match &self.heading {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        return Ok(new_head);
    }

    pub fn add_new_head(&mut self, new_head: Position, drop_tail: bool) {
        self.positions.push_front(new_head);

        if drop_tail {
            self.positions.pop_back();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{direction::Direction, snake::Snake};

    #[test]
    fn construct_new_instance() {
        println!("{:?}", Snake::new((3, 3), Direction::Left));
    }

    #[test]
    fn handle_key_press_arrow_up_goes_up() {
        let mut snake = Snake::new((3, 3), Direction::Left);

        snake.handle_key_press(String::from("ArrowUp"));

        assert_eq!(snake.next_heading, Direction::Up);
    }

    #[test]
    fn handle_key_press_w_goes_up() {
        let mut snake = Snake::new((3, 3), Direction::Left);

        snake.handle_key_press(String::from("W"));

        assert_eq!(snake.next_heading, Direction::Up);
    }

    #[test]
    fn handle_key_press_arrow_down_goes_down() {
        let mut snake = Snake::new((3, 3), Direction::Left);

        snake.handle_key_press(String::from("ArrowDown"));

        assert_eq!(snake.next_heading, Direction::Down);
    }

    #[test]
    fn handle_key_press_s_goes_down() {
        let mut snake = Snake::new((3, 3), Direction::Left);

        snake.handle_key_press(String::from("S"));

        assert_eq!(snake.next_heading, Direction::Down);
    }

    #[test]
    fn handle_key_press_arrow_right_does_nothing() {
        let mut snake = Snake::new((3, 3), Direction::Left);

        snake.handle_key_press(String::from("ArrowRight"));

        assert_eq!(snake.next_heading, Direction::Left);
    }

    #[test]
    fn handle_key_press_d_does_nothing() {
        let mut snake = Snake::new((3, 3), Direction::Left);

        snake.handle_key_press(String::from("D"));

        assert_eq!(snake.next_heading, Direction::Left);
    }
}
