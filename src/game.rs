use rand::Rng;
use yew::{html, Component, Properties};

use crate::{
    cell::Cell, collidable::Collidable, direction::Direction, food::Food, game_status::GameStatus,
    position::Position, snake::Snake,
};

#[derive(Debug, Properties, PartialEq)]
pub struct Game {
    size: usize,
    snake: Snake,
    food: Food,
    status: GameStatus,
}

impl Component for Game {
    type Message = ();

    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Game::new(20)
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        // TODO!
        html!(<div>{ "GAME" }</div>)
    }
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            snake: Snake::new(((size - 3).max(0), size / 2), Direction::Left),
            food: Food::new((2.min(size - 1), size / 2)),
            status: GameStatus::Paused,
        }
    }

    pub fn handle_key_press(&mut self, key_code: &str) {
        if key_code == "Space" {
            match self.status {
                GameStatus::Playing => self.pause(),
                GameStatus::Paused => self.unpause(),
                GameStatus::GameOver => {
                    // todo!("start new game?")
                }
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

    fn score(&self) -> u32 {
        self.snake.body_length()
    }

    fn get_cell(&self, position: Position) -> Option<Cell> {
        if self.is_out_of_bounds(position) {
            return None;
        }

        Some(match position {
            (x, y) if self.snake.is_head_position((x, y)) => Cell::SnakeHead,
            (x, y) if self.snake.check_for_collision(&(x, y)) => Cell::SnakeBody,
            (x, y) if self.food.check_for_collision(&(x, y)) => Cell::Food,
            _ => Cell::Empty,
        })
    }

    fn is_out_of_bounds(&self, position: Position) -> bool {
        let (x, y) = position;
        x > (self.size - 1) || y > (self.size - 1)
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
        let available_positions = self.get_available_positions();
        available_positions[rand::thread_rng().gen_range(0..(available_positions.len() + 1))]
    }
}
