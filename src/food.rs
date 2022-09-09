use std::collections::VecDeque;

use crate::{collidable::Collidable, position::Position};

#[derive(Debug)]
pub struct Food {
    positions: VecDeque<Position>,
}

impl Food {
    pub fn new(starting_position: Position) -> Self {
        Self {
            positions: [starting_position].into_iter().collect(),
        }
    }
}

impl Collidable for Food {
    fn check_for_collision(&self, position: &Position) -> bool {
        self.positions.contains(position)
    }
}
