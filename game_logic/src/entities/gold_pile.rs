use rand::Rng;
use simple_vector2::Vector2;
use crate::drawable::Drawable;
use crate::entities::Entity;
use crate::map::Map;
use crate::placeable::Placeable;

pub struct GoldPile {
    pub worth: u32,
    pub location: Vector2<usize>,
}

impl GoldPile {
    pub fn new(worth: u32, location: Vector2<usize>,) -> GoldPile {
        GoldPile { worth, location }
    }
    
    pub fn is_greater_loot_pile(&self) -> bool {
        self.worth > 100
    }
}

impl Placeable for GoldPile {
    fn place(map: &Map, rng: &mut impl Rng) -> Option<Vector2<usize>> {
        Self::random_tile_in_random_room(map, rng)
    }
}

impl Drawable for GoldPile {
    fn as_char(&self) -> char {
        if self.is_greater_loot_pile() {
            'G'
        } else {
            'g'
        }
    }

    fn world_space_position(&self) -> Vector2<usize> {
        self.location
    }
}
