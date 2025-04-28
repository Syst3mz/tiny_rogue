use crate::spawn_table_entry::SpawnTableEntry;

pub trait Placeable {
    fn get_spawn_table() -> &'static [SpawnTableEntry<Self>] where Self: Sized; 
}