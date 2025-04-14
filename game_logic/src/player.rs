use simple_vector2::Vector2;
use shared::constants::{SCREEN_SIZE};

pub struct Player {
    pub position: Vector2<usize>,
}

impl Player {
    pub fn new() -> Player {
        Self {
            position: Vector2::new(0, 0),
        }
    }
}