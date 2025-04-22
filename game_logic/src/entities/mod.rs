use alloc::boxed::Box;
use crate::damage_taker::DamageTaker;
use crate::entities::gold_pile::GoldPile;
use crate::entities::player::Player;
use crate::entities::rat::Rat;
use crate::player_hunter::PlayerTracker;

pub mod gold_pile;
pub mod player;
pub mod rat;

pub enum Entity {
    Player(Player),
    GoldPile(GoldPile),
    Rat(Rat)
}

impl Entity {
    pub fn as_player(&self) -> Option<&Player> {
        match self {
            Entity::Player(p) => Some(p),
            _ => None
        }
    }

    pub fn as_player_mut(&mut self) -> Option<&mut Player> {
        match self {
            Entity::Player(p) => Some(p),
            _ => None
        }
    }
}