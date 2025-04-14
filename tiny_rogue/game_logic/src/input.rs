pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B
}
pub trait Input {
    fn button_down(&mut self) -> Option<Button>;
}