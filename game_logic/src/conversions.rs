use simple_vector2::Vector2;

pub trait Vector2ToTuple<T> {
    fn to_tuple(self) -> (T, T);
}

impl<T> Vector2ToTuple<T> for Vector2<T> {
    fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}