use core::array::IntoIter;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};
use simple_vector2::Vector2;
use crate::grid_direction::GridDirection;

pub struct Array2d< const TOTAL_ELEMENTS: usize, T> {
    data: [T; TOTAL_ELEMENTS],
    size: Vector2<usize>,
}

impl<const TOTAL_ELEMENTS: usize, T> IntoIterator for Array2d<TOTAL_ELEMENTS, T> {
    type Item = T;
    type IntoIter = IntoIter<T, TOTAL_ELEMENTS>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[allow(dead_code)]
impl<const TOTAL_ELEMENTS: usize, T> Array2d<TOTAL_ELEMENTS, T> {
    pub fn index_at(&self, at: Vector2<usize>) -> usize {
        at.y * self.size.x + at.x
    }

    pub fn row(&self, row: usize) -> &[T]{
        let row_start = row * self.size.x;
        &self.data[row_start..row_start + self.size.x]
    }

    pub fn get_pixel_mut(&mut self, at: Vector2<usize>) -> Option<&mut T> {
        let index = self.index_at(at);
        self.data.get_mut(index)
    }

    pub fn get_pixel(&self, at: Vector2<usize>) -> Option<&T> {
        self.data.get(self.index_at(at))
    }

    pub fn width(&self) -> usize {
        self.size.x
    }

    pub fn height(&self) -> usize {
        self.size.y
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// Get the cardinally adjacent neighbors, neighbors which are off grid are represented as None
    pub fn cardinal_neighbors(&self, from: Vector2<usize>) -> [Option<Vector2<usize>>; 4] {
        let mut answer = [None; 4];

        if from.x != 0 {
            answer[GridDirection::West.to_index()] = Some(Vector2::new(from.x - 1, from.y));
        }

        if from.y != 0 {
            answer[GridDirection::North.to_index()] = Some(Vector2::new(from.x, from.y - 1));
        }

        if from.x < self.size.x - 1 {
            answer[GridDirection::East.to_index()] = Some(Vector2::new(from.x + 1, from.y));
        }

        if from.y < self.size.y - 1 {
            answer[GridDirection::South.to_index()] = Some(Vector2::new(from.x, from.y + 1));
        }

        answer
    }
}

impl<const TOTAL_ELEMENTS: usize, T: Copy> Array2d<TOTAL_ELEMENTS, T> {
    pub fn new(data: [T; TOTAL_ELEMENTS], size: Vector2<usize>) -> Self {
        Self {
            data,
            size,
        }
    }
    pub fn clear(&mut self, with: T) {
        self.data = [with; TOTAL_ELEMENTS];
    }
    pub fn copy_from_slice_offset(&mut self, offset_by: usize, source: &[T]) {
        let dst_slice = &mut self.data[offset_by..offset_by + source.len()];
        dst_slice.copy_from_slice(source);
    }
}

impl<const TOTAL_ELEMENTS: usize, T> Index<usize> for Array2d<TOTAL_ELEMENTS, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const TOTAL_ELEMENTS: usize, T> IndexMut<usize> for Array2d<TOTAL_ELEMENTS, T> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

