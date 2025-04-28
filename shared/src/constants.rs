
use simple_vector2::Vector2;

pub const FONT_METRICS: Vector2<usize> = Vector2::new(8, 11);
pub const SCREEN_SIZE: Vector2<usize> = Vector2::new(400 / FONT_METRICS.x, 240 / FONT_METRICS.y);
pub const DRAWING_BUFFER_SIZE: usize = SCREEN_SIZE.x * SCREEN_SIZE.y;

pub const MAP_SIZE: Vector2<usize> = Vector2::new(80, 24);
pub const MAP_BUFFER_SIZE: usize = MAP_SIZE.x * MAP_SIZE.y;
pub const MAZE_SIZE: Vector2<usize> = Vector2::new((MAP_SIZE.x - 2) / 2, (MAP_SIZE.y - 2) / 2);
pub const MAZE_BUFFER_SIZE: usize = MAZE_SIZE.x * MAZE_SIZE.y;

pub const MAX_ROOM_SIZE: Vector2<usize> = Vector2::new(MAP_SIZE.x / 3, MAP_SIZE.y / 3);
pub const MIN_ROOM_SIZE: Vector2<usize> = Vector2::new(5, 5);
// The buffer size also happens to the area of the map. 
// I figure I want about 80 percent of the map to be room space.
pub const DESIRED_ROOM_COVERAGE: usize = 4 * (MAZE_BUFFER_SIZE / 5);
pub const PLACEMENT_ATTEMPTS: usize = 100;
pub const MAX_STAIRS: usize = 5;
pub const LEVEL_BASE_ITEM_BUDGET: usize = 100;
pub const LEVEL_BASE_ENEMY_BUDGET: usize = 30;