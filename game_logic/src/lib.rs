#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::{format, vec};
use alloc::vec::Vec;
use hashbrown::HashSet;
use rand::prelude::{IndexedRandom, SmallRng};
use rand::{Rng, RngCore, SeedableRng};
use simple_vector2::Vector2;
use shared::constants::SCREEN_SIZE;
use shared::logger::Logger;
use crate::camera::Camera;
use crate::damage_taker::DamageTaker;
use crate::drawable::Drawable;
use crate::entities::item::Item;
use crate::entities::player::Player;
use crate::entities::enemy::{Attack, Enemy};
use crate::input::{Button, Input};
use crate::located::Located;
use crate::map::{Map, Tile};
use crate::placement_director::PlacementDirector;
use crate::renderer::Renderer;

pub mod renderer;
pub mod conversions;
pub mod input;
mod map;
mod camera;
mod rectangle;
mod entities;
mod drawable;
mod damage_taker;
mod placement_director;
mod located;

pub static LOGGER: Logger = Logger::new();

pub struct Game<Render: Renderer, Inp: Input> {
    pub renderer: Render,
    pub input: Inp,
    map: Map,
    rng: SmallRng,
    player: Player,
    enemies: Vec<Located<Enemy>>,
    items: Vec<Located<Item>>,
    camera: Camera,
}
impl<Render: Renderer, Inp: Input> Game<Render, Inp> {
    pub fn new(renderer: Render, input: Inp, seed: u64) -> Game<Render, Inp> {
        Game {
            renderer,
            input,
            map: Map::new(),
            rng: SmallRng::seed_from_u64(seed),
            player: Player::new(Vector2::new(1, 1)),
            enemies: vec![],
            items: vec![],
            camera: Camera::new(Vector2::new(0, 0)),
        }
    }
    
    fn make_level(&mut self) {
        self.enemies.clear();
        self.items.clear();
        self.map.generate_map(&mut self.rng);
        
        let (items, enemies) = PlacementDirector::new(60, &mut self.rng).place_all(&self.map);
        self.items = items;
        self.enemies = enemies;
        
        
        let Some(starting_room) = self.map.rooms.choose(&mut self.rng) else {return};
        self.player.position = starting_room.top_left + Vector2::new(1, 1);
        self.camera.track_player(self.player.position);
    }

    pub fn init(&mut self) {
        self.make_level()
    }

    fn write_map_to_renderer(&mut self) {
        let camera_box = self.camera.get_camera_bounds();
        let camera_box_end = camera_box.bottom_right();

        for row in camera_box.top_left.y..camera_box_end.y {
            for column in camera_box.top_left.x..camera_box_end.x {
                let desired_pixel_location = Vector2::new(column, row);

                let Some(desired_pixel) = self.map.grid.get_pixel(desired_pixel_location) else {
                    LOGGER.error(format!("Tried to read from {} and it doesn't exist.", desired_pixel_location));
                    continue;
                };

                let Some(transformed_camera_position) = self.camera.transform_world_to_camera(desired_pixel_location) else {
                    LOGGER.error(format!("Tried to transform {} to camera space it is out of bounds.", desired_pixel_location));
                    continue;
                };

                self.renderer.set_pixel(transformed_camera_position, desired_pixel.as_char());
            }
        }
    }
    
    
    fn transform_player_position(&self) -> Option<Vector2<usize>> {
        let player_position = self.player.position;
        let answer = self.camera.transform_world_to_camera(player_position);
        
        if answer.is_none() {
            LOGGER.error(format!("Tried to transform player position ({}) to camera space it is out of bounds.", player_position));
        }
        
        answer
    }
    
    fn write_player_to_renderer(&mut self) -> Option<()> {
        self.renderer.set_pixel(self.transform_player_position()?, '@');
        Some(())
    }
    
    fn write_lines(&mut self, lines: &[impl AsRef<str>], reversed: bool) {
        let mut row = if reversed { 
            SCREEN_SIZE.y - 1
        } else {
            0
        };
        
        if reversed {
            for line in lines.iter().rev() {
                self.renderer.print(line, Vector2::new(0, row));
                row = row.saturating_sub(1);
            }
        } else {
            for line in lines.iter() {
                self.renderer.print(line, Vector2::new(0, row));
                row = row.saturating_add(1)
            }
        }
    }
    
    fn write_player_stats_to_renderer(&mut self) -> Option<()>{
        let transformed_position = self.transform_player_position()?;
        let reversed = transformed_position.y < SCREEN_SIZE.y / 2;

        let lines = [
            format!("Health: {}, Score: {}", self.player.health, self.player.score),
        ];

        self.write_lines(&lines, reversed);

        Some(())
    }
    fn update_enemies(&mut self) {
        let impassable = self.collect_impassable();
        
        for enemy_index in 0..self.enemies.len() {
            self.enemies[enemy_index].update(&self.map, &mut self.rng, &mut self.player, &impassable)
        }
    }
    
    fn go_to_next_floor(&mut self) {
        let Some(tile) = self.map.get_tile_at(self.player.position) else { return; };
        let new_floor_seed = match tile {
            Tile::Stairs(seed) => seed,
            _ => return,
        };

        self.rng = SmallRng::seed_from_u64(new_floor_seed);
        self.make_level();
    }
    
    fn do_player_contact(&mut self) {
        for enemy in (0..self.enemies.len()).rev() {
            if self.player.position != self.enemies[enemy].world_space_position() { 
                continue;
            }

            self.enemies[enemy].take_damage(self.player.attack);

            if self.enemies[enemy].is_dead() {
                self.enemies.remove(enemy);
            }
        }
        
        
        for item_index in (0..self.items.len()).rev() {
            if self.items[item_index].world_space_position() == self.player.position {
                let item = self.items.remove(item_index);
                item.unwrap().consume(&mut self.player)
            }
        }
    }
    
    fn write_entities_to_renderer(&mut self) {
        for items in self.items.iter() {
            let Some(transformed_item_position) = self.camera.transform_world_to_camera(items.world_space_position()) else { continue };
            self.renderer.set_pixel(transformed_item_position, items.as_char());
        }

        for enemy in self.enemies.iter() {
            let Some(enemy_position) = self.camera.transform_world_to_camera(enemy.world_space_position()) else { continue };
            self.renderer.set_pixel(enemy_position, enemy.as_char());
        }
    }
    
    fn collect_impassable(&self) -> HashSet<Vector2<usize>> {
        HashSet::from_iter(self.enemies.iter().map(|x| x.world_space_position()))
    }

    pub fn update(&mut self) {
        let player_input = self.input.button_down();
        if let Some(player_input) = player_input {
            // drop the player down a floor if they are on the stairs
            match player_input { 
                Button::Down => self.go_to_next_floor(),
                _ => {}
            }
            
            self.player.move_player(&self.map, player_input, self.collect_impassable());
            self.camera.track_player(self.player.position);

            self.update_enemies();
            
            self.do_player_contact();   
        }
        
        self.renderer.clear(None);
        self.write_map_to_renderer();
        self.write_entities_to_renderer();
        self.write_player_to_renderer();
        self.write_player_stats_to_renderer();
    }
}