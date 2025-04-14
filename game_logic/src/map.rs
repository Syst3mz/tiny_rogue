use alloc::vec::Vec;
use core::ops::Index;
use rand::prelude::SliceRandom;
use rand::Rng;
use shared::array_2d::Array2d;
use shared::constants::{MAP_BUFFER_SIZE, MAP_SIZE};
use simple_vector2::Vector2;
use crate::map::Tile::{Floor, Wall};

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Wall,
    Floor,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        match self {
            Wall => true,
            _ => false,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Wall => '#',
            Floor => '.',
        }
    }
}

pub struct Map {
    pub grid: Array2d<MAP_BUFFER_SIZE, Tile>
}

impl Map {
    pub fn new() -> Self {
        Self {
            grid: Array2d::new([Floor; MAP_BUFFER_SIZE], MAP_SIZE),
        }
    }

    // Generates a maze using the Iterative randomized Prim's algorithm (without stack, without sets)
    // more info at: https://en.wikipedia.org/wiki/Maze_generation_algorithm#Iterative_randomized_Prim's_algorithm_(without_stack,_without_sets)
    pub fn generate_maze(&mut self, rng: &mut impl Rng) {
        self.grid.clear(Wall);

        // make checkerboard...
        for row in 1..=self.grid.height() - 1 {
            for column in 1..=self.grid.width() - 1 {
                let index =  self.grid.index_at(Vector2::new(column, row));
                let parity = index % 2 == 1;
                if parity {
                    self.grid[index] = Tile::Floor;
                }
            }
        }


    }
}