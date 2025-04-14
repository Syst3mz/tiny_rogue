
use simple_vector2::Vector2;

pub const FONT_METRICS: Vector2<usize> = Vector2::new(8, 11);
pub const SCREEN_SIZE: Vector2<usize> = Vector2::new(400 / FONT_METRICS.x, 240 / FONT_METRICS.y);
pub const MAP_SIZE: Vector2<usize> = Vector2::new(80, 24);

pub const DRAWING_BUFFER_SIZE: usize = SCREEN_SIZE.x * SCREEN_SIZE.y;