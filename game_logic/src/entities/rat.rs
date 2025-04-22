use rand::Rng;
use simple_vector2::Vector2;
use crate::damage_taker::DamageTaker;
use crate::drawable::Drawable;
use crate::entities::Entity;
use crate::map::Map;
use crate::placeable::Placeable;
use crate::player_hunter::PlayerTracker;

pub struct Rat {
    position: Vector2<usize>,
    health: u16,
    pub attack: u16
}

impl Rat {
    pub fn new(location: Vector2<usize>, health: u16) -> Self {
        Self {
            position: location,
            health,
            attack: 4,
        }
    }
}
impl Drawable for Rat {
    fn as_char(&self) -> char {
        'r'
    }

    fn world_space_position(&self) -> Vector2<usize> {
        self.position
    }
}

impl DamageTaker for Rat {
    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn take_damage(&mut self, damage: u16) {
        self.health = self.health.saturating_sub(damage)
    }
}



impl PlayerTracker for Rat {
    fn move_self(&mut self, map: &Map, _: &mut impl Rng, player_position: Vector2<usize>) {
        let desired_move = Self::cardinal_move_nearer_to(self.position, player_position);
        if !map.tile_can_be_moved_into(desired_move) { 
            return;
        }
        
        self.position = desired_move;
    }
}

impl Placeable for Rat {
    fn place(map: &Map, rng: &mut impl Rng) -> Option<Vector2<usize>> {
        Rat::random_tile_in_random_room(map, rng)
    }
}