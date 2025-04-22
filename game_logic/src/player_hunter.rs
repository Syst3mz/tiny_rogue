use rand::Rng;
use simple_vector2::Vector2;
use shared::constants::MAP_SIZE;
use crate::map::Map;

pub trait PlayerTracker {
    fn move_self(&mut self, map: &Map, rng: &mut impl Rng, player_position: Vector2<usize>);
    fn cardinal_move_nearer_to(from: Vector2<usize>, to: Vector2<usize>) -> Vector2<usize> {
        if from.x > to.x {
            return Vector2::new((from.x + 1).min(MAP_SIZE.x), from.y)
        } 
        
        if from.x < to.x {
            return Vector2::new(from.x.saturating_sub(1), from.y)
        }

        if from.y < to.x {
            return Vector2::new(from.x, (from.y + 1).min(MAP_SIZE.y))
        } 
        
        if from.x > to.x {
            return Vector2::new(from.x, from.y.saturating_sub(1))
        }
        
        Vector2::new(from.x, from.y)
    }
}