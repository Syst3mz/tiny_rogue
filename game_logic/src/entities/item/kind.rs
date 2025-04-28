use alloc::format;
use crate::entities::player::Player;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    GoldPile(u32),
    Sword(u16),
    Armor(u16),
    HealingPotion(u16),
}

impl Kind {
    pub fn as_char(&self) -> char {
        match self {
            Kind::GoldPile(worth) => {
                if *worth > 100 {
                    'G'
                } else {
                    'g'
                }
            }
            Kind::Sword(_) | Kind::Armor(_) => 'I',
            Kind::HealingPotion(_) => 'P'
        }
    }

    pub fn consume(self, player: &mut Player) {
        match self {
            Kind::GoldPile(g) => {
                player.increase_score(g);
            },
            Kind::Sword(s) => {
                player.attack += s;
                player.log_message(format!("+{} attack!", s));
            },
            Kind::Armor(a) => {
                player.defense += a;
                player.log_message(format!("+{} defense!", a));
            },
            Kind::HealingPotion(h) => {
                player.health += h;
                player.log_message(format!("+{} health!", h));
            }
        }
    }
}