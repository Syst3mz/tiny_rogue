use core::fmt::Display;
use core::ops::{Add, Mul};
use rand::distr::uniform::SampleUniform;
use rand::Rng;
use simple_vector2::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle<T> {
    pub top_left: Vector2<T>,
    pub size: Vector2<T>,
}

impl<T: Display> Display for Rectangle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}x{} @ {})", self.size.x, self.size.y, self.top_left)
    }
}

impl<T> Rectangle<T> {
    pub fn new(top_left: Vector2<T>, size: Vector2<T>) -> Self {
        Self { top_left, size }
    }
}

impl<T: Clone+SampleUniform+Add<Output=T>+PartialOrd> Rectangle<T> {
    pub fn rand_inside(&self, rng: &mut impl Rng) -> Vector2<T> {
        Vector2::new(
            rng.random_range(self.top_left.x.clone()..=self.top_left.x.clone() + self.size.x.clone()), 
            rng.random_range(self.top_left.y.clone()..=self.top_left.y.clone() + self.size.y.clone())
        )
    }
}

impl Rectangle<usize> {
    pub fn shrink(&self, by: usize) -> Rectangle<usize> {
        Self {
            top_left: self.top_left + Vector2::new(by, by),
            size: self.size - Vector2::new(2 * by, 2 * by),
        }
    }
}

#[allow(dead_code)]
impl<T: Add<Output=T>+Clone> Rectangle<T> {
    pub fn top_right(&self) -> Vector2<T> {
        Vector2::new(self.top_left.x.clone() + self.size.x.clone(), self.top_left.y.clone())
    }
    pub fn bottom_right(&self) -> Vector2<T> {
        Vector2::new(self.top_left.x.clone() + self.size.x.clone(), self.top_left.y.clone() + self.size.y.clone())
    }
    pub fn bottom_left(&self) -> Vector2<T> {
        Vector2::new(self.top_left.x.clone(), self.top_left.y.clone() + self.size.y.clone())
    }
}
impl<T: Add<Output=T>+Mul<Output=T>+Clone> Rectangle<T> {
    pub fn area(&self) -> T {
        self.size.x.clone() * self.size.y.clone()
    }
}

#[allow(dead_code)]
impl<T: Clone+PartialOrd+Add<Output=T>> Rectangle<T> {
    pub fn contains(&self, other: &Self) -> bool {
        let self_bottom_right = self.bottom_right();
        let other_bottom_right = other.bottom_right();

        self.top_left.x <= other.top_left.x
            && self.top_left.y <= other.top_left.y
            && self_bottom_right.x >= other_bottom_right.x
            && self_bottom_right.y >= other_bottom_right.y
    }
    pub fn within(&self, other: &Self) -> bool {
        other.contains(self)
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::SmallRng;
    use rand::SeedableRng;
    use shared::constants::MAP_SIZE;
    use super::*;

    #[test]
    fn test_contains() {
        let map = Rectangle::new(Vector2::new(0, 0), MAP_SIZE);
        let room = Rectangle::new(Vector2::new(0, 0), Vector2::new(25, 5));
        assert!(map.contains(&room));
    }
    
    #[test]
    fn test_neg_contains() {
        let map = Rectangle::new(Vector2::new(0, 0), MAP_SIZE - Vector2::new(1, 1));
        let room = Rectangle::new(Vector2::new(62, 19), Vector2::new(25, 5));
        assert!(!map.contains(&room));
    }

    #[test]
    fn test_neg_within() {
        let map = Rectangle::new(Vector2::new(0, 0), MAP_SIZE - Vector2::new(1, 1));
        let room = Rectangle::new(Vector2::new(62, 19), Vector2::new(25, 5));
        assert!(!map.within(&room));
    }
    
    #[test]
    fn rand_inside() {
        let min = Vector2::new(0, 0);
        let max = Vector2::new(16, 16);
        let room = Rectangle::new(min, max - min);
        let mut rng = SmallRng::seed_from_u64(0);
        
        for _ in 0..1000 {
            let rand_inside = room.rand_inside(&mut rng);
            assert!(min.x <= rand_inside.x && rand_inside.x <= max.x);
            assert!(min.y <= rand_inside.y && rand_inside.y <= max.y);
        }
    }
    
    #[test]
    fn shrink() {
        let room = Rectangle::new(Vector2::new(0, 0), Vector2::new(16, 16));
        let shrinked = room.shrink(1);
        assert_eq!(shrinked.top_left, Vector2::new(1, 1));
        assert_eq!(shrinked.size, Vector2::new(14, 14));
    }
}