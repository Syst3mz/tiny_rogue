#[derive(Debug, Clone, Copy)]
pub struct SpawnTableEntry<T> {
    pub to_spawn: T,
    pub weight: u32,
    pub cost: usize
}