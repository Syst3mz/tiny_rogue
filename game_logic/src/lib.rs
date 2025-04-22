#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::{format, vec};
use alloc::vec::Vec;
use rand::prelude::{IndexedRandom, SmallRng};
use rand::{Rng, RngCore, SeedableRng};
use simple_vector2::Vector2;
use shared::constants::{MAP_SIZE, SCREEN_SIZE};
use shared::logger::Logger;
use crate::camera::Camera;
use crate::damage_taker::DamageTaker;
use crate::drawable::Drawable;
use crate::entities::Entity;
use crate::entities::gold_pile::GoldPile;
use crate::entities::player::Player;
use crate::entities::rat::Rat;
use crate::input::{Button, Input};
use crate::map::{Map, Tile};
use crate::placeable::Placeable;
use crate::player_hunter::PlayerTracker;
use crate::renderer::Renderer;

pub mod renderer;
pub mod conversions;
pub mod input;
mod map;
mod camera;
mod rectangle;
mod placeable;
mod entities;
mod drawable;
mod player_hunter;
mod damage_taker;

pub static LOGGER: Logger = Logger::new();

pub struct Game<Render: Renderer, Inp: Input> {
    pub renderer: Render,
    pub input: Inp,
    map: Map,
    rng: SmallRng,
    player: Player,
    rats: Vec<Rat>,
    gold_piles: Vec<GoldPile>,
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
            rats: vec![],
            gold_piles: vec![],
            camera: Camera::new(Vector2::new(0, 0)),
        }
    }
    
    fn make_level(&mut self) {
        self.map.generate_map(&mut self.rng);
        self.gold_piles.push(GoldPile::new(60, Vector2::new(2, 1)));
        for _ in 0..30 {
            let Some(gold_pile) = GoldPile::place(&self.map, &mut self.rng) else { continue };
            self.gold_piles.push(GoldPile::new(self.rng.random_range(30..301), gold_pile))
        }

        for _ in 0..30 {
            let Some(rat) = Rat::place(&self.map, &mut self.rng) else { continue };
            self.rats.push(Rat::new(rat, self.rng.random_range(3..6)))
        }
        
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
    fn move_entities(&mut self) {
        /*for rat_index in 0..self.rats.len() {
            self.rats[rat_index].move_self(&self.map, &mut self.rng, self.player.position)
        }*/
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
        for rat_index in (0..self.rats.len()).rev() {
            if self.player.position == self.rats[rat_index].world_space_position() { 
                self.rats[rat_index].take_damage(self.player.attack);
                self.player.take_damage(self.rats[rat_index].attack);
                
                if self.rats[rat_index].is_dead() { 
                    self.rats.remove(rat_index);
                }
            }
        }
        
        
        for gold_pile_index in (0..self.gold_piles.len()).rev() {
            if self.gold_piles[gold_pile_index].world_space_position() == self.player.position {
                self.player.score += self.gold_piles.remove(gold_pile_index).worth
            }
        }
    }
    
    fn write_entities_to_renderer(&mut self) {
        for rat in self.rats.iter() {
            self.renderer.set_pixel(rat.world_space_position(), rat.as_char());
        }

        for gold_pile in self.gold_piles.iter() {
            self.renderer.set_pixel(gold_pile.world_space_position(), gold_pile.as_char());
        }
    }

    pub fn update(&mut self) {
        let player_input = self.input.button_down();
        if let Some(player_input) = player_input {
            // drop the player down a floor if they are on the stairs
            match player_input { 
                Button::Down => self.go_to_next_floor(),
                _ => {}
            }
            
            self.player.move_player(&self.map, player_input);

            self.move_entities();
            self.do_player_contact();   
        }
        
        self.renderer.clear(None);
        self.write_map_to_renderer();
        self.write_entities_to_renderer();
        self.write_player_to_renderer();
        self.write_player_stats_to_renderer();
    }
}