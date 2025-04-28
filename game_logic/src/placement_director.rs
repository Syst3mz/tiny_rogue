use crate::LOGGER;
use crate::map::Map;
use crate::Vector2;
use alloc::vec::Vec;
use core::marker::PhantomData;
use rand::prelude::IndexedRandom;
use rand::Rng;
use shared::constants::{MAP_SIZE, PLACEMENT_ATTEMPTS};
use crate::located::Located;
use crate::placeable::Placeable;

pub enum PlacementMode {
    RandomInRoom,
    RandomFloorTile,
}

pub struct PlacementDirector<'a, T: Rng, P: Placeable> {
    budget: usize,
    rng: &'a mut T,
    phantom_p: PhantomData<P>,
    mode: PlacementMode
}

impl<'a, T: Rng, P: Placeable+Clone+'static> PlacementDirector<'a, T, P> {
    pub fn new(budget: usize, rng: &'a mut T, mode: PlacementMode) -> Self {
        Self {
            budget,
            rng,
            phantom_p: PhantomData::default(),
            mode,
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

    fn random_from_table_in_budget(&mut self) -> Option<(P, usize)> {
        let table = P::get_spawn_table();
        let mut attempts = PLACEMENT_ATTEMPTS;
        while attempts > 0 {
            let Ok(chosen) = table.choose_weighted(&mut self.rng,|x| x.weight) else {
                LOGGER.error("Unable to choose from a random spawn table. It's probably empty.");
                return None;
            };
            
            if self.budget >= chosen.cost {
                return Some((chosen.to_spawn.clone(), chosen.cost))
            }
            
            attempts -= 1;
        }
        
        None
    }
    
    fn random_floor_tile(&mut self, map: &Map) -> Option<Vector2<usize>> {
        fn random_tile_on_map(rng: &mut impl Rng) -> Vector2<usize> {
            Vector2::new(
                rng.random_range(0..MAP_SIZE.x),
                rng.random_range(0..MAP_SIZE.y)
            )
        }
        
        let mut attempts = PLACEMENT_ATTEMPTS;
        while attempts > 0 {
            let random_tile = random_tile_on_map(self.rng);
            if map.tile_can_be_moved_into(random_tile) {
                return Some(random_tile);
            }
            
            attempts -= 1;
        }
        
        None
    }
    
    pub fn place_all(mut self, on: &Map) -> Vec<Located<P>> {
        let mut placed = Vec::new();
        
        while self.budget > 0 {
            let budget = self.budget;
            
            let Some((unplaced, cost)) = self.random_from_table_in_budget() else { 
                break 
            };
            
            self.budget -= cost;
            
            let position = match self.mode {
                PlacementMode::RandomInRoom => self.random_tile_in_random_room(on),
                PlacementMode::RandomFloorTile => self.random_floor_tile(on),
            };
            
            let Some(position) = position else {
                // refund the spawn budget used this system was unable to place.
                self.budget = budget;
                
                LOGGER.error("Unable to place item.");
                continue;
            };
            
            placed.push(Located::new(unplaced, position));
        }
 
        placed
    }
}