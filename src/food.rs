use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{collidable::Collidable, position::Position};

#[derive(Debug, Serialize, Deserialize)]
pub struct Food {
    positions: VecDeque<Position>,
}

impl Food {
    pub fn new(position: Position) -> Self {
        Self {
            positions: [position].into_iter().collect(),
        }
    }
}

impl Collidable for Food {
    fn check_for_collision(&self, position: &Position) -> bool {
        self.positions.contains(position)
    }
}
