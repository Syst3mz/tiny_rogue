use simple_vector2::Vector2;

pub trait Drawable {
    /// The character used to represent this entity
    fn as_char(&self) -> char;
    
    fn world_space_position(&self) -> Vector2<usize>;
}