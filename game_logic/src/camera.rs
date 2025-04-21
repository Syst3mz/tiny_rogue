use simple_vector2::Vector2;
use shared::constants::{MAP_SIZE, SCREEN_SIZE};
use crate::rectangle::Rectangle;

pub struct Camera {
    pub position: Vector2<usize>
}

impl Camera {
    pub fn new(position: Vector2<usize>) -> Self {
        Self {
            position
        }
    }

    pub fn track_player(&mut self, player_position: Vector2<usize>) {
        let half_view = SCREEN_SIZE / 2;

        // Try to center the camera on the player
        let mut x = player_position.x.saturating_sub(half_view.x);
        let mut y = player_position.y.saturating_sub(half_view.y);

        // Clamp so the camera doesn't exceed the map bounds
        x = x.min(MAP_SIZE.x.saturating_sub(SCREEN_SIZE.x));
        y = y.min(MAP_SIZE.y.saturating_sub(SCREEN_SIZE.y));

        self.position = Vector2::new(x, y);
    }
    pub fn get_camera_bounds(&self) -> Rectangle<usize> {
       let mut maximal_bounding_box = Rectangle::new(self.position, SCREEN_SIZE);

        maximal_bounding_box.size.x = maximal_bounding_box.size.x.min(MAP_SIZE.x - self.position.x);
        maximal_bounding_box.size.y = maximal_bounding_box.size.y.min(MAP_SIZE.y - self.position.y);

        maximal_bounding_box
    }

    pub fn transform_world_to_camera(&self, world_position: Vector2<usize>) -> Option<Vector2<usize>> {
        let x = world_position.x.checked_sub(self.position.x)?;
        let y = world_position.y.checked_sub(self.position.y)?;

        if x >= SCREEN_SIZE.x || y >= SCREEN_SIZE.y {
            return None;
        }

        Some(Vector2::new(x, y))
    }
}