use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashSet;
use rand::prelude::IndexedRandom;
use rand::Rng;
use simple_vector2::Vector2;
use shared::array_2d::Array2d;
use shared::constants::{MAZE_BUFFER_SIZE, MAZE_SIZE};
// Map of walls, Up, Right, Down, Left
#[derive(Copy, Clone, Debug)]
pub struct Cell {
    walls: [bool; 4]
}

impl Default for Cell {
    fn default() -> Self {
        Cell { walls: [true; 4] }
    }
}
pub struct Maze {
    maze: Array2d<MAZE_BUFFER_SIZE, Cell>
}

impl Maze {
    pub fn new() -> Self {
        Self {
            maze: Array2d::new([Cell::default(); MAZE_BUFFER_SIZE], MAZE_SIZE),
        }
    }

    fn random_position(&self, rng: &mut impl Rng) -> Vector2<usize> {
        Vector2::new(
            rng.random_range(0..MAZE_SIZE.x),
            rng.random_range(0..MAZE_SIZE.y),
        )
    }

    fn generate(&mut self, rng: &mut impl Rng) {
        let initial_position = self.random_position(rng);
        let mut visited = hashbrown::HashSet::from([initial_position]);
        let mut stack = vec![initial_position];
        while !stack.is_empty() {
            let current_cell = stack.pop().unwrap();

            let mut unvisited_neighbors = self.maze.cardinal_neighbors(current_cell);
            unvisited_neighbors.retain(|x| !visited.contains(x));

            let Some(neighbor) = unvisited_neighbors.choose(rng);

        }
    }
}

