#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;
use simple_vector2::Vector2;
use crate::renderer::Renderer;

pub mod renderer;
pub mod conversions;

pub struct Game<Render: Renderer> {
    pub renderer: Render,
}

impl<Render: Renderer> Game<Render> {
    pub fn new(renderer: Render) -> Game<Render> {
        Game {
            renderer,
        }
    }

    pub fn draw_border(&mut self) {
        self.renderer.set_pixel(Vector2::new(0, 0), 'A');
        self.renderer.line(Vector2::new(0, 1), Vector2::new(10, 10), 'B');
        self.renderer.border(Vector2::new(0, 0), self.renderer.last_valid_indices(), '#')
    }

    pub fn init(&mut self) {
        self.draw_border()
    }


}