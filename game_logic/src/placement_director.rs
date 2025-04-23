use crate::{Item, LOGGER};
use crate::Enemy;
use crate::map::Map;
use crate::Vector2;
use alloc::vec::Vec;
use rand::prelude::IndexedRandom;
use rand::Rng;
use rand::rngs::SmallRng;
use shared::constants::PLACEMENT_ATTEMPTS;
use crate::located::Located;

pub struct PlacementDirector<'a, T: Rng> {
    budget: usize,
    rng: &'a mut T
}

impl<'a, T: Rng> PlacementDirector<'a, T> {
    pub fn new(budget: usize, rng: &'a mut T) -> Self {
        Self {
            budget,
            rng
        }
    }

    fn random_tile_in_random_room(&mut self, map: &Map) -> Option<Vector2<usize>> {
        let mut attemps = PLACEMENT_ATTEMPTS;
        while attemps > 0 {
            let Some(chosen_room) = map.rooms.choose(self.rng) else { continue };
            let chosen_position = chosen_room.shrink(1).rand_inside(self.rng);

            if map.tile_can_be_moved_into(chosen_position) {
                return Some(chosen_position);
            }

            attemps -= 1;
        }

        LOGGER.error("Unable to generate a random tile in a random room.");
        None
    }

    pub fn place_all(mut self, on: &Map) -> (Vec<Located<Item>>, Vec<Located<Enemy>>) {
        let mut items = Vec::new();
        let mut enemies = Vec::new();
        
        let mut item_budget = 2 * (self.budget / 3);
        let mut enemy_budget = self.budget - item_budget;

        while item_budget > 0 {
            let unplaced_item = Item::random(&mut self.rng);
            let Some(position) = self.random_tile_in_random_room(on) else {
                LOGGER.error("Unable to place item.");
                continue;
            };
            
            items.push(Located::new(unplaced_item, position));
            item_budget -= 1
        }

        while enemy_budget > 0 {
            let unplaced_enemy = Enemy::random(&mut self.rng);
            let Some(position) = self.random_tile_in_random_room(on) else {
                LOGGER.error("Unable to place item.");
                continue;
            };

            enemies.push(Located::new(unplaced_enemy, position));
            enemy_budget -= 1
        }
                

        (items, enemies)
    }
}