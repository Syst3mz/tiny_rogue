use simple_vector2::Vector2;

pub struct Player {
    pub position: Vector2<usize>,
    pub health: u16,
}

impl Player {
    pub fn new(position: Vector2<usize>) -> Player {
        Self {
            position,
            health: 100,
        }
    }
    
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
    
    pub fn take_damage(&mut self, damage: u16) {
        if damage >= self.health { 
            self.health = 0;
        }
        else {
            self.health -= damage;
        }
    }
}