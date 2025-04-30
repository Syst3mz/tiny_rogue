use alloc::{format, vec};
use alloc::vec::Vec;
use hashbrown::HashSet;
use line_drawing::Bresenham;
use rand::prelude::IndexedRandom;
use rand::Rng;
use shared::array_2d::Array2d;
use shared::constants::{DESIRED_ROOM_COVERAGE, MAP_BUFFER_SIZE, MAP_SIZE, MAX_ROOM_SIZE, MAX_STAIRS, MIN_ROOM_SIZE, PLACEMENT_ATTEMPTS, SQUARE_PLAYER_STAIR_SPAWN_RADIUS};
use simple_vector2::Vector2;
use crate::conversions::Vector2ToTuple;
use crate::rectangle::Rectangle;
use crate::LOGGER;
use crate::map::Tile::{Floor, Wall};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    Stairs(u64)
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
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Stairs(_) => 'V',
        }
    }
}

pub struct Map {
    pub grid: Array2d<MAP_BUFFER_SIZE, Tile>,
    pub rooms: Vec<Rectangle<usize>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            grid: Array2d::new([Floor; MAP_BUFFER_SIZE], MAP_SIZE),
            rooms: Vec::new(),
        }
    }
    
    pub fn get_tile_at(&self, point: Vector2<usize>) -> Option<Tile> {
        self.grid.get_pixel(point).map(|x| *x)
    }
    
    pub fn tile_can_be_moved_into(&self, point: Vector2<usize>) -> bool {
        let Some(tile) = self.grid.get_pixel(point) else { return false; };
        !tile.is_wall()
    }
    
    fn random_point_on_map(rng: &mut impl Rng) -> Vector2<usize> {
        Vector2::new(
            rng.random_range(0..MAP_SIZE.x),
            rng.random_range(0..MAP_SIZE.y),
        )
    }
    
    fn random_odd_point(rng: &mut impl Rng) -> Vector2<usize> {
        let mut seed_point = Self::random_point_on_map(rng);
        while seed_point.x % 2 == 0 || seed_point.y % 2 == 0 {
            seed_point = Self::random_point_on_map(rng);
        }
        seed_point
    }
    
    fn get_frontier_points_of(point: Vector2<usize>) -> Vec<Vector2<usize>> {
        let mut frontier_points = Vec::new();
        if point.x >= 2 { 
            frontier_points.push(point - Vector2::new(2, 0));
        }

        if point.y >= 2 {
            frontier_points.push(point - Vector2::new(0, 2));
        }

        if point.x < MAP_SIZE.x - 2 {
            frontier_points.push(point + Vector2::new(2, 0));
        }
        
        if point.y < MAP_SIZE.y - 2 {
            frontier_points.push(point + Vector2::new(0, 2));
        }
        
        frontier_points
    }
    
    fn get_point_between(a: Vector2<usize>, b: Vector2<usize>) -> Vector2<usize> {
        (a + b) / 2
    }
    pub fn can_see(&self, from: Vector2<usize>, to: Vector2<usize>) -> bool {
        let points_on_ray = Bresenham::new(
            from.map(|x| x as isize).to_tuple(), 
            to.map(|x| x as isize).to_tuple()
        );

        for (x, y) in points_on_ray {
            let position = Vector2::new(x as usize, y as usize);
            if !self.tile_can_be_moved_into(position) {
                return false;
            }
        }
        
        true
    }
    
    pub fn generate_map(&mut self, rng: &mut impl Rng) {
        self.grid.clear(Wall);
        self.rooms.clear();
        self.generate_maze(rng);
        self.generate_rooms(rng);
    }

    fn place_stair(&mut self, player_position: &Vector2<usize>, rng: &mut impl Rng) {
        let mut attempts = PLACEMENT_ATTEMPTS;
        while attempts > 0 {
            let Some(room) = self.rooms.choose(rng) else { continue; };
            let random_position_in_shrank_room = room.shrink(1).rand_inside(rng);

            if random_position_in_shrank_room.square_distance(player_position) >= SQUARE_PLAYER_STAIR_SPAWN_RADIUS {
                let stair_index = self.grid.index_at(random_position_in_shrank_room);
                self.grid[stair_index] = Tile::Stairs(rng.random());
                return;
            }

            attempts -= 1;
        }            
    }
    
    pub fn place_stairs(&mut self, player_position: &Vector2<usize>, rng: &mut impl Rng) {
        for _ in 0..MAX_STAIRS {
            self.place_stair(player_position, rng)
        }
    }
    
    pub fn generate_maze(&mut self, rng: &mut impl Rng) {
        // Choose the initial cell, mark it as visited and push it to the stack
        let seed_point = Self::random_odd_point(rng);
        let mut visited = HashSet::new();
        visited.insert(seed_point);
        let mut stack = vec![seed_point];
        
        // While the stack is not empty
        // Pop a cell from the stack and make it a current cell
        while let Some(current_cell) = stack.pop() {
            let index_of_point = self.grid.index_at(current_cell);
            self.grid[index_of_point] = Floor;
            
            let mut neighbors = Self::get_frontier_points_of(current_cell);
            neighbors.retain(|neighbor| !visited.contains(neighbor));
            
            if neighbors.is_empty() {
               continue; 
            }
            
            // Push the current cell to the stack
            stack.push(current_cell);
            
            // Choose one of the unvisited neighbors
            let neighbor = *neighbors.choose(rng).unwrap();
            
            // Remove the wall between the current cell and the chosen cell
            let point_between_index = self.grid.index_at(Self::get_point_between(current_cell, neighbor));
            self.grid[point_between_index] = Floor;
            
            stack.push(neighbor);
            visited.insert(neighbor);
        }
    }
    
    /// Attempts to place a room on the map.
    /// Returns true if the room was placed, false otherwise.
    pub fn place_room(&mut self, room: Rectangle<usize>) -> bool {
        if room.area() < 25 {
            LOGGER.error(format!("Room {} is too small...aborting placement.", room));
            return false;
        }
        
        let map = Rectangle::new(Vector2::new(0, 0), MAP_SIZE - Vector2::new(1, 1));
        if !map.contains(&room) {
            LOGGER.error(format!("Room {} doesn't fit on the map! (placement aborted)", room));
            return false;
        }
        
        let interior = Rectangle::new(
            room.top_left + Vector2::new(1, 1),
            room.size - Vector2::new(1, 1),
        );
        let interior_bottom_right = interior.bottom_right();
        
        for row in interior.top_left.y..interior_bottom_right.y {
            for column in interior.top_left.x..interior_bottom_right.x {
                let tile_index = self.grid.index_at(Vector2::new(column, row));
                self.grid[tile_index] = Floor;
            }
        }
        
        self.rooms.push(room);
        true
    }
    
    fn room_size_mapped(rng: &mut impl Rng) -> Vector2<usize> {
        fn x_to_the_fourth(x: f32) -> f32 {
            let x_squared = x * x;
            x_squared * x_squared
        }
        let x:f32 = x_to_the_fourth(rng.random()) * MAX_ROOM_SIZE.x as f32;
        let y:f32 = x_to_the_fourth(rng.random()) * MAX_ROOM_SIZE.y as f32;
        let x = (x as usize).max(MIN_ROOM_SIZE.x);
        let y = (y as usize).max(MIN_ROOM_SIZE.y);
        
        Vector2::new(x, y)
    }
    
    pub fn generate_and_place_room(&mut self, rng: &mut impl Rng) {
        let mut attempts = PLACEMENT_ATTEMPTS;
        while attempts > 0 {
            let room = Rectangle::new(
                Self::random_point_on_map(rng),
                /*Vector2::new(
                    rng.random_range(MIN_ROOM_SIZE.x..MAX_ROOM_SIZE.x),
                    rng.random_range(MIN_ROOM_SIZE.y..MAX_ROOM_SIZE.y)
                ),*/
                Self::room_size_mapped(rng),
            );

            if self.place_room(room) {
                break;
            }
            
            attempts -= 1;
        }
        
        if attempts == 0 { 
            LOGGER.error(format!("Failed to place a room on the map after {} attempts!", PLACEMENT_ATTEMPTS));
        }
    }
    
    fn area_of_rooms(&self) -> usize {
        self.rooms.iter().map(|x| x.area()).sum()
    }
    pub fn generate_rooms(&mut self, rng: &mut impl Rng) {
        while self.area_of_rooms() < DESIRED_ROOM_COVERAGE {
            self.generate_and_place_room(rng)
        }
    }
}