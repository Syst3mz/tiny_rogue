use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashSet;
use rand::Rng;
use simple_vector2::Vector2;
use shared::constants::MAP_SIZE;
use crate::damage_taker::DamageTaker;
use crate::drawable::Drawable;
use crate::entities::player::Player;
use crate::located::Located;
use crate::map::Map;

/// The square distance from the player (ignoring LOS) that the enemy will start to track the player from.
const ENEMY_AGRO_RANGE: usize = 8 * 8;
pub enum Attack {
    Melee(u16),
    Ranged(u16, u8)
}

pub struct Enemy {
    name: String,
    display_char: char,
    pub health: u16,
    armor: u16,
    pub attack: Attack,
    pub worth: u32
}

impl Enemy {
    pub fn new(name: impl AsRef<str>, display_char: char, health: u16, armor: u16, attack: Attack, worth: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            display_char,
            health,
            armor,
            attack,
            worth,
        }
    }
    
    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.random_range(0..3) { 
            0 => Enemy::new("Rat", 'r', rng.random_range(1..=4), 0, Attack::Melee(rng.random_range(2..=4)), 1),
            1 => Enemy::new("Zombie", 'z', rng.random_range(4..=12), 2, Attack::Melee(rng.random_range(4..=8)), 2),
            2 => Enemy::new("Zombie Archer", 'a', rng.random_range(2..=6), 1, Attack::Ranged(rng.random_range(2..=6), 3), 4),
            3 => Enemy::new("Lich", 'L', rng.random_range(32..=64), 5, Attack::Ranged(rng.random_range(12..=36), 4), 8),
            _ => unreachable!()
        }
    }
}

impl Located<Enemy> {
    pub fn can_hit_player(&self, player_position: Vector2<usize>) -> bool {
        match self.attack {
            Attack::Melee(_) => self.position == player_position,
            Attack::Ranged(_, range) => {
                let square_range = (range as usize) * (range as usize);
                self.position.square_distance(&player_position) <= square_range
            }
        }
    }

    fn cardinal_move_nearer_to(&self, to: Vector2<usize>) -> Vector2<usize> {
        if self.position.x < to.x {
            Vector2::new((self.position.x + 1).min(MAP_SIZE.x), self.position.y)
        } else if self.position.x > to.x {
            Vector2::new(self.position.x.saturating_sub(1), self.position.y)
        } else if self.position.y < to.y {
            Vector2::new(self.position.x, (self.position.y + 1).min(MAP_SIZE.y))
        } else if self.position.y > to.y {
            Vector2::new(self.position.x, self.position.y.saturating_sub(1))
        } else {
            self.position
        }
    }
    
    fn move_towards_player(&mut self, player_position: Vector2<usize>, map: &Map, impassable_tiles: &HashSet<Vector2<usize>>) {
        let desired_move = self.cardinal_move_nearer_to(player_position);
        if !map.tile_can_be_moved_into(desired_move) || impassable_tiles.contains(&desired_move) {
            return;
        }

        self.position = desired_move;
    }

    pub fn update(&mut self, map: &Map, _: &mut impl Rng, player: &mut Player, impassable_tiles: &HashSet<Vector2<usize>>) {
        // todo: Maybe let make enemies move more randomly?
        // todo: better pathfinding
        
        if self.position.square_distance(&player.position) <= ENEMY_AGRO_RANGE {
            self.move_towards_player(player.position, map, impassable_tiles);
        }
        
        if !self.can_hit_player(player.position) {
            return;
        }
        
        let damage = match self.attack {
            Attack::Melee(d)  | Attack::Ranged(d, _) => d
        };
        
        player.take_damage(damage);
    }
}

impl Drawable for Located<Enemy> {
    fn as_char(&self) -> char { self.display_char }

    fn world_space_position(&self) -> Vector2<usize> {
        self.position
    }
}

impl DamageTaker for Enemy {
    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn take_damage(&mut self, damage: u16) {
        self.health = self.health.saturating_sub(damage.saturating_sub(self.armor));
    }
}