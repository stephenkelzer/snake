use crate::{collidable::Collidable, position::Position};

#[derive(Debug, PartialEq)]
pub struct Food {
    positions: Vec<Position>,
}

impl Food {
    pub fn new(position: Position) -> Self {
        Self {
            positions: [position].to_vec(),
        }
    }
}

impl Collidable for Food {
    fn check_for_collision(&self, position: &Position) -> bool {
        self.positions.contains(position)
    }
}
