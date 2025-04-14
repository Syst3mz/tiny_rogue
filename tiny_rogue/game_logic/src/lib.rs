#![cfg_attr(not(test), no_std)]

use simple_vector2::Vector2;
use crate::input::{Button, Input};
use crate::renderer::Renderer;

pub mod renderer;
pub mod conversions;
pub mod input;
mod world;

pub struct Game<Render: Renderer, Inp: Input> {
    pub renderer: Render,
    pub input: Inp,
    player: Vector2<usize>,
    bounds: Vector2<usize>,
}

impl<Render: Renderer, Inp: Input> Game<Render, Inp> {
    pub fn new(renderer: Render, input: Inp) -> Game<Render, Inp> {
        let bounds = renderer.bounds();
        Game {
            renderer,
            input,
            player: Vector2::new(1, 1),
            bounds,
        }
    }

    pub fn draw_border(&mut self) {
        self.renderer.border(Vector2::new(0, 0), self.renderer.last_valid_indices(), '#')
    }

    pub fn init(&mut self) {
        // self.draw_border()
    }

    pub fn update(&mut self) {
        self.move_player();


        self.renderer.clear(None);
        self.draw_border();
        self.renderer.set_pixel(self.player, '@')
    }

    fn move_player(&mut self) {
        let Some(button) = self.input.button_down() else { return; };
        match button {
            Button::Up => self.player.y = self.player.y.saturating_sub(1),
            Button::Down => self.player.y = self.player.y.saturating_add(1),
            Button::Left => self.player.x = self.player.x.saturating_sub(1),
            Button::Right => self.player.x = self.player.x.saturating_add(1),
            _ => return,
        }

        // no need to test if position is under zero since rust will have a moment for me. Also,
        // the saturating will prevent it.

        self.player.x = self.player.x.min(self.bounds.x);
        self.player.y = self.player.y.min(self.bounds.y);
    }
}