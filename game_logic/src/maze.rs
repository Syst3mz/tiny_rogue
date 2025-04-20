use alloc::vec;
use alloc::vec::Vec;
use core::ops::Index;
use hashbrown::HashSet;
use rand::prelude::IndexedRandom;
use rand::Rng;
use simple_vector2::Vector2;
use shared::array_2d::Array2d;
use shared::constants::{MAZE_BUFFER_SIZE, MAZE_SIZE};
use shared::grid_direction::GridDirection;

// Map of walls, North, East, South, West
type Cell = [bool; 4];
trait CellOps {
    fn new() -> Cell;
    fn direction(&self, grid_direction: GridDirection) -> bool;
    fn direction_mut(&mut self, grid_direction: GridDirection) -> &mut bool;
}

impl CellOps for Cell {
    fn new() -> Cell {
        [true; 4]
    }

    fn direction(&self, grid_direction: GridDirection) -> bool {
        self[grid_direction.to_index()]
    }

    fn direction_mut(&mut self, grid_direction: GridDirection) -> &mut bool {
        &mut self[grid_direction.to_index()]
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
        /*let initial_position = self.random_position(rng);
        let mut visited = hashbrown::HashSet::from([initial_position]);
        let mut stack = vec![initial_position];
        while !stack.is_empty() {
            let current_cell = stack.pop().unwrap();

            let neighbors = self.maze.cardinal_neighbors(current_cell);
            let neighbors = neighbors
                .into_iter()
                .enumerate()
                .filter_map(|x| );


            let Some(neighbor) = unvisited_neighbors.choose(rng) else { continue; };

        }*/
    }
}

