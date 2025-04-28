mod kind;

use simple_vector2::Vector2;
use crate::drawable::Drawable;
use crate::entities::item::kind::Kind;
use crate::entities::player::Player;
use crate::located::Located;
use crate::placeable::Placeable;
use crate::spawn_table_entry::SpawnTableEntry;

#[derive(Clone, Debug)]
pub struct Item {
    pub kind: Kind,
    pub name: &'static str,
}

impl Item {
    pub const fn new(kind: Kind, name: &'static str) -> Item {
        Item { kind, name }
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

const SPAWN_TABLE: [SpawnTableEntry<Item>; 8] = [
    SpawnTableEntry {
        to_spawn: Item::new(Kind::Sword(2), "Iron Sword"),
        weight: 20,
        cost: 10,
    },
    SpawnTableEntry {
        to_spawn: Item::new(Kind::Sword(4), "Enchanted Sword"),
        weight: 4,
        cost: 100,
    },
    SpawnTableEntry {
        to_spawn: Item::new(Kind::Armor(2), "Iron Armor"),
        weight: 10,
        cost: 20,
    },
    SpawnTableEntry {
        to_spawn: Item::new(Kind::Armor(4), "Enchanted Armor"),
        weight: 2,
        cost: 200,
    },
    SpawnTableEntry {
        to_spawn: Item::new(Kind::GoldPile(500), "Large Gold Pile"),
        weight: 2,
        cost: 300,
    },
    SpawnTableEntry {
        to_spawn: Item::new(Kind::GoldPile(1500), "Dragon's Horde"),
        weight: 1,
        cost: 600,
    },
    SpawnTableEntry {
        to_spawn: Item::new(Kind::HealingPotion(25), "Healing Potion"),
        weight: 10,
        cost: 4,
    },
    SpawnTableEntry {
        to_spawn: Item::new(Kind::HealingPotion(100), "Major Healing Potion"),
        weight: 1,
        cost: 40,
    }
];


impl Placeable for Item {
    fn get_spawn_table() -> &'static [SpawnTableEntry<Self>]
    where
        Self: Sized
    {
        &SPAWN_TABLE
    }
}