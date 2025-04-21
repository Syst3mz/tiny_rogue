#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::format;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use simple_vector2::Vector2;
use shared::constants::{MAP_SIZE, SCREEN_SIZE};
use shared::logger::Logger;
use crate::camera::Camera;
use crate::input::{Button, Input};
use crate::map::Map;
use crate::player::Player;
use crate::renderer::Renderer;

pub mod renderer;
pub mod conversions;
pub mod input;
mod map;
mod player;
mod camera;
mod rectangle;
mod placeable;

pub static LOGGER: Logger = Logger::new();

pub struct Game<Render: Renderer, Inp: Input> {
    pub renderer: Render,
    pub input: Inp,
    map: Map,
    rng: SmallRng,
    player: Player,
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
            camera: Camera::new(Vector2::new(0, 0)),
        }
    }

    pub fn init(&mut self) {
        self.map.generate_map(&mut self.rng)
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
        let answer = self.camera.transform_world_to_camera(self.player.position);
        
        if answer.is_none() {
            LOGGER.error(format!("Tried to transform player position ({}) to camera space it is out of bounds.", self.player.position));
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
            format!("Health: {}", self.player.health),
        ];
        
        self.write_lines(&lines, reversed);
        
        Some(())
    }

    pub fn update(&mut self) {
        self.move_player();

        self.renderer.clear(None);
        self.write_map_to_renderer();
        self.write_player_to_renderer();
        self.write_player_stats_to_renderer();
    }

    fn player_move_desire(&mut self) -> Option<Vector2<usize>> {
        let Some(button) = self.input.button_down() else { return None; };
        let mut desire = self.player.position;

        // since I just move the player, we invert directions.
        match button {
            Button::Up => desire.y = desire.y.saturating_sub(1),
            Button::Down => desire.y = desire.y.saturating_add(1),
            Button::Left => desire.x = desire.x.saturating_sub(1),
            Button::Right => desire.x = desire.x.saturating_add(1),
            _ => return None,
        };

        // no need to test if position is under zero since rust will have a moment for me. Also,
        // the saturating will prevent it.

        desire.x = desire.x.min(MAP_SIZE.x - 1);
        desire.y = desire.y.min(MAP_SIZE.y - 1);
        Some(desire)
    }

    fn move_player(&mut self) {
        let Some(desire) = self.player_move_desire() else { return; };
        if let Some(tile_at) = self.map.grid.get_pixel(desire) {
            if tile_at.is_wall() {
                return
            }
            
            self.player.position = desire;
            self.camera.track_player(self.player.position);
        }
    }
}