use rand::Rng;
use simple_vector2::Vector2;
use crate::map::Map;

pub trait Placeable {
    fn place(map: &Map, rng: &mut impl Rng) -> Vector2<usize>;
}