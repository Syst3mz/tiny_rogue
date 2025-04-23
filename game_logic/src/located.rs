use core::ops::{Deref, DerefMut};
use simple_vector2::Vector2;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Located<T> {
    pub element: T,
    pub position: Vector2<usize>
}

impl<T> Located<T> {
    pub fn new(element: T, position: Vector2<usize>) -> Self {
        Self { element, position }
    }
    
    pub fn unwrap(self) -> T {
        self.element
    }
}

impl<T> Deref for Located<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.element
    }
} 

impl<T> DerefMut for Located<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}