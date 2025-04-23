use simple_vector2::Vector2;
use shared::constants::MAP_SIZE;
use crate::damage_taker::DamageTaker;
use crate::drawable::Drawable;
use crate::input;
use crate::input::Button;
use crate::map::Map;

pub struct Player {
    pub position: Vector2<usize>,
    pub health: u16,
    pub score: u32,
    pub attack: u16,
    pub defense: u16,
    levels_completed: u32,
}

impl Player {
    pub fn new(position: Vector2<usize>) -> Player {
        Self {
            position,
            health: 100,
            score: 0,
            attack: 3,
            defense: 2,
            levels_completed: 0,
        }
    }

    fn desired_move(&self, button: Button) -> Option<Vector2<usize>> {
        let mut desire = self.position;

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

    pub fn move_player(&mut self, on: &Map, pressed_button: Button, impassible_positions: impl IntoIterator<Item=Vector2<usize>>) -> Option<()> {
        let desire = self.desired_move(pressed_button)?;
        
        if !on.tile_can_be_moved_into(desire) {
            return None;
        }
        
        for impassable_position in impassible_positions {
            if impassable_position == desire { 
                return None;
            }
        }

        self.position = desire;
        Some(())
    }
    
    pub fn increase_score(&mut self, by: u32) {
        self.score += (self.levels_completed / 2).max(1) * by;
    }
}

impl Drawable for Player {
    fn as_char(&self) -> char {
        '@'
    }

    fn world_space_position(&self) -> Vector2<usize> {
        self.position
    }
}

impl DamageTaker for Player {
    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn take_damage(&mut self, damage: u16) {
        // apply my armour
        let damage = damage.saturating_sub(self.defense);

        if damage >= self.health {
            self.health = 0;
        }
        else {
            self.health -= damage;
        }
    }
}

