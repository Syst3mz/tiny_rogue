use rand::prelude::IndexedRandom;
use rand::Rng;
use simple_vector2::Vector2;
use shared::constants::{MAP_SIZE, PLACEMENT_ATTEMPTS};
use crate::map::{Map, Tile};
use crate::rectangle::Rectangle;

pub trait Placeable {
    fn place(map: &Map, rng: &mut impl Rng) -> Option<Vector2<usize>>;
    fn random_tile_in_random_room(map: &Map, rng: &mut impl Rng) -> Option<Vector2<usize>> {
        let chosen_room= map.rooms.choose(rng)?;
        
        
        Some(chosen_room.shrink().rand_inside(rng))
    }
    
    fn random_floor_tile(map: &Map, rng: &mut impl Rng ) -> Option<Vector2<usize>> {
        let map_as_rect = Rectangle::new(Vector2::new(0, 0), MAP_SIZE);
        let mut attempts = PLACEMENT_ATTEMPTS;
        while attempts > 0 {
            let tile = map_as_rect.rand_inside(rng);
            if map.grid.get_pixel(tile)? == &Tile::Floor { 
                return Some(tile);
            }
            
            attempts -= 1;
        }
        
        None
    }
}