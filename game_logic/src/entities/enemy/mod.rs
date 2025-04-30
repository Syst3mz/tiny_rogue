pub mod kind;

use alloc::string::{String, ToString};
use hashbrown::HashSet;
use rand::prelude::IteratorRandom;
use rand::Rng;
use simple_vector2::Vector2;
use shared::constants::MAP_SIZE;
use crate::damage_taker::DamageTaker;
use crate::drawable::Drawable;
use crate::entities::enemy::kind::Kind;
use crate::entities::player::Player;
use crate::located::Located;
use crate::LOGGER;
use crate::map::Map;
use crate::placeable::Placeable;
use crate::spawn_table_entry::SpawnTableEntry;

/// The square distance from the player (ignoring LOS) that the enemy will start to track the player from.
const ENEMY_AGRO_RANGE: usize = 8 * 8;
#[derive(Debug, Clone, Copy)]
pub enum Attack {
    Melee(u16),
    Ranged(u16, u8)
}


#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    kind: Kind,
    pub health: u16,
    armor: u16,
    pub attack: Attack,
    pub worth: u32
}

impl Enemy {
    pub const fn new(kind: Kind, health: u16, armor: u16, attack: Attack, worth: u32) -> Self {
        Self {
            kind,
            health,
            armor,
            attack,
            worth,
        }
    }

    pub fn name(&self) -> String {
        self.kind.to_string()
    }
}

impl Located<Enemy> {
    pub fn can_hit_player(&self, player_position: Vector2<usize>, map: &Map) -> bool {
        match self.attack {
            Attack::Melee(_) => self.position == player_position,
            Attack::Ranged(_, range) => {
                let square_range = (range as usize) * (range as usize);
                let in_range = self.position.square_distance(&player_position) <= square_range;
                if !in_range { return false }

                map.can_see(self.position, player_position)
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

    fn can_move_into_tile(map: &Map, desired_move: Vector2<usize>, impassable_tiles: &HashSet<Vector2<usize>>) -> bool {
        map.tile_can_be_moved_into(desired_move) && !impassable_tiles.contains(&desired_move)
    }

    fn move_towards_player(&mut self, player_position: Vector2<usize>, map: &Map, impassable_tiles: &HashSet<Vector2<usize>>) {
        let desired_move = self.cardinal_move_nearer_to(player_position);

        if !Self::can_move_into_tile(map, desired_move, impassable_tiles) { return; }

        self.position = desired_move;
    }

    fn move_self(&mut self, map: &Map, rng: &mut impl Rng, player: &mut Player, impassable_tiles: &HashSet<Vector2<usize>>) {
        if self.position.square_distance(&player.position) > ENEMY_AGRO_RANGE {
            return;
        }

        if map.can_see(self.position, player.position) {
            self.move_towards_player(player.position, map, impassable_tiles);
        }
        else {
            let next_position = map.grid.cardinal_neighbors(self.position)
                .into_iter()
                .filter_map(|x| x)
                .choose(rng);

            let Some(desired_move) = next_position else {
                LOGGER.debug("Enemy has no valid moves!");
                return;
            };

            if !Self::can_move_into_tile(map, desired_move, impassable_tiles) { return; }

            self.position = desired_move;
        }
    }

    fn attack_player(&mut self, player: &mut Player, map: &Map) {
        if !self.can_hit_player(player.position, map) {
            return;
        }

        match self.attack {
            Attack::Melee(d) | Attack::Ranged(d, _) => player.take_damage(d, self.kind.to_string()),
        }
    }

    pub fn update(&mut self, map: &Map, rng: &mut impl Rng, player: &mut Player, impassable_tiles: &HashSet<Vector2<usize>>) {
        self.move_self(map, rng, player, impassable_tiles);
        self.attack_player(player, map)
    }
}

impl Drawable for Located<Enemy> {
    fn as_char(&self) -> char { self.kind.display_char() }

    fn world_space_position(&self) -> Vector2<usize> {
        self.position
    }
}

impl DamageTaker for Enemy {
    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn take_damage(&mut self, damage: u16, _: impl AsRef<str>) {
        self.health = self.health.saturating_sub(damage.saturating_sub(self.armor));
        if Kind::Lich(true) == self.kind && self.health == 0 {
            self.health = 70;
            self.kind = Kind::Lich(false);
        }
    }
}

const SPAWN_TABLE: [SpawnTableEntry<Enemy>; 4] = [
    SpawnTableEntry {
        to_spawn: Enemy::new(Kind::Rat, 2, 0, Attack::Melee(3), 1),
        weight: 20,
        cost: 1,
    },
    SpawnTableEntry {
        to_spawn: Enemy::new(Kind::Zombie, 8, 2, Attack::Melee(6), 10),
        weight: 10,
        cost: 4,
    },
    SpawnTableEntry {
        to_spawn: Enemy::new(Kind::ZombieArcher, 8, 3, Attack::Ranged(6, 3), 20),
        weight: 5,
        cost: 32,
    },
    SpawnTableEntry {
        to_spawn: Enemy::new(Kind::Lich(true), 70, 12, Attack::Melee(36), 1),
        weight: 2,
        cost: 128,
    },
    
];
impl Placeable for Enemy {
    fn get_spawn_table() -> &'static [SpawnTableEntry<Self>] {
        &SPAWN_TABLE
    }
}