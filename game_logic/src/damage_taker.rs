pub trait DamageTaker {
    fn is_dead(&self) -> bool;
    fn take_damage(&mut self, damage: u16);
}