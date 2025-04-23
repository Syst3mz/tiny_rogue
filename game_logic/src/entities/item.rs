use alloc::string::{String, ToString};
use rand::prelude::IndexedRandom;
use rand::Rng;
use simple_vector2::Vector2;
use crate::drawable::Drawable;
use crate::entities::player::Player;
use crate::located::Located;

const UNIQUE_ITEMS: [(ItemKind, &'static str); 6] = [
    (ItemKind::Sword(2), "Iron Sword"),
    (ItemKind::Sword(4), "Enchanted Sword"),
    (ItemKind::Armor(2), "Iron Armor"),
    (ItemKind::Armor(4), "Enchanted Sword"),
    (ItemKind::GoldPile(500), "Large Pile of Gold"),
    (ItemKind::Sword(1500), "Dragon's Horde")
];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    GoldPile(u32),
    Sword(u16),
    Armor(u16),
    HealingPotion(u16),
}

impl ItemKind {
    pub fn as_char(&self) -> char {
        match self {
            ItemKind::GoldPile(worth) => {
                if *worth > 100 {
                    'G'
                } else {
                    'g'
                }
            }
            ItemKind::Sword(_) | ItemKind::Armor(_) => 'I',
            ItemKind::HealingPotion(_) => 'P' 
        }
    }
    
    pub fn consume(self, player: &mut Player) {
        match self {
            ItemKind::GoldPile(g) => player.score += g,
            ItemKind::Sword(s) => player.attack += s,
            ItemKind::Armor(a) => player.defense += a,
            ItemKind::HealingPotion(h) => player.health += h,
        }
    }
}

pub struct Item {
    pub kind: ItemKind,
    pub name: String,
}

impl Item {
    pub fn new(kind: ItemKind, name: impl AsRef<str>) -> Item {
        Item { kind, name: name.as_ref().to_string() }
    }
    
    fn random_unique(rng: &mut impl Rng) -> Item {
        let (kind, name) = UNIQUE_ITEMS.choose(rng).unwrap();
        Item::new(*kind, name)
    }
    
    fn random_non_unique(rng: &mut impl Rng) -> Item {
        if rng.random_bool(0.5) {
            Item::new(ItemKind::GoldPile(rng.random_range(1..100)), "Pile of Gold".to_string())
        } else {
            Item::new(ItemKind::HealingPotion(rng.random_range(20..100)), "Potion of Healing".to_string())
        }
    }
    pub fn random(rng: &mut impl Rng) -> Item {
        if rng.random_bool(0.5) {
            Self::random_unique(rng)
        } else {
            Self::random_non_unique(rng)
        }
    }
    
    pub fn consume(self, player: &mut Player) {
        self.kind.consume(player);
    }
}

impl Drawable for Located<Item> {
    fn as_char(&self) -> char {
        self.kind.as_char()
    }

    fn world_space_position(&self) -> Vector2<usize> {
        self.position
    }
}