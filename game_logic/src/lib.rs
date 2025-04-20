#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use simple_vector2::Vector2;
use shared::constants::{MAP_SIZE, SCREEN_SIZE};
use shared::logger::Logger;
use crate::input::{Button, Input};
use crate::map::Map;
use crate::player::Player;
use crate::renderer::Renderer;

pub mod renderer;
pub mod conversions;
pub mod input;
mod world;
mod map;
mod player;
mod maze;

pub struct Game<Render: Renderer, Inp: Input> {
    pub renderer: Render,
    pub input: Inp,
    map: Map,
    rng: SmallRng,
    pub logger: Logger,
    player: Player,
}

impl<Render: Renderer, Inp: Input> Game<Render, Inp> {
    pub fn new(renderer: Render, input: Inp, seed: u64) -> Game<Render, Inp> {
        Game {
            renderer,
            input,
            map: Map::new(),
            rng: SmallRng::seed_from_u64(seed),
            logger: Logger::new(),
            player: Player::new(),
        }
    }

    pub fn init(&mut self) {
        self.map.generate_maze(&mut self.rng)
    }

    fn write_map_to_renderer(&mut self) {
        let half_screen_size = SCREEN_SIZE / 2;
        let start_of_cam = Vector2::new(
            self.player.position.x.saturating_sub(half_screen_size.x),
            self.player.position.y.saturating_sub(half_screen_size.y),
        );
        let end_of_cam = self.player.position + half_screen_size;
        let end_of_cam = Vector2::new(
            end_of_cam.x.min(MAP_SIZE.x - 1),
            end_of_cam.y.min(MAP_SIZE.y - 1)
        );

        let mut display_row = 0;
        for row in start_of_cam.y..=end_of_cam.y {
            let offset = self.renderer.width() * display_row;
            let row_contents = &self.map.grid.row(row)[..SCREEN_SIZE.x];
            let row_contents:Vec<char> = row_contents.iter().map(|x| x.as_char()).collect();

            self.renderer.copy_from_slice_offset(offset, &row_contents);

            display_row += 1;
        }
    }

    pub fn update(&mut self) {
        self.move_player();


        self.renderer.clear(None);

        self.write_map_to_renderer();
        self.renderer.set_pixel(SCREEN_SIZE / 2, '@')

    }

    fn player_move_desire(&mut self) -> Option<Vector2<usize>> {
        let Some(button) = self.input.button_down() else { return None; };
        let mut desire = self.player.position;

        // since I just move the player, we invert directions.
        match button {
            Button::Down => desire.y = desire.y.saturating_sub(1),
            Button::Up => desire.y = desire.y.saturating_add(1),
            Button::Right => desire.x = desire.x.saturating_sub(1),
            Button::Left => desire.x = desire.x.saturating_add(1),
            _ => return None,
        };

        // no need to test if position is under zero since rust will have a moment for me. Also,
        // the saturating will prevent it.

        desire.x = desire.x.clamp(0, MAP_SIZE.x - 1);
        desire.y = desire.y.clamp(0, MAP_SIZE.y - 1);
        Some(desire)
    }

    fn move_player(&mut self) {
        let Some(desire) = self.player_move_desire() else { return; };
        if let Some(tile_at) = self.map.grid.get_pixel(desire) {
            if tile_at.is_wall() {
                return
            }

            self.player.position = desire;
        }

    }
}