use crate::position::Position;

pub trait Collidable {
    fn check_for_collision(&self, _: &Position) -> bool;
}
