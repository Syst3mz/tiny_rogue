use core::array::IntoIter;
use core::slice::{Iter, IterMut};
use simple_vector2::Vector2;
use crate::constants::SCREEN_SIZE;

pub struct Array2d<const ROWS: usize, const COLUMNS: usize, const TOTAL_ELEMENTS: usize, T> {
    data: [T; TOTAL_ELEMENTS],
}

impl<const ROWS: usize, const COLUMNS: usize, const TOTAL_ELEMENTS: usize, T> IntoIterator for Array2d<ROWS, COLUMNS, TOTAL_ELEMENTS, T> {
    type Item = T;
    type IntoIter = IntoIter<T, TOTAL_ELEMENTS>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[allow(dead_code)]
impl<const ROWS: usize, const COLUMNS: usize, const TOTAL_ELEMENTS: usize, T> Array2d<ROWS, COLUMNS, TOTAL_ELEMENTS, T> {
    pub fn new(data: [T; TOTAL_ELEMENTS]) -> Self {
        Self { data }
    }

    pub fn index(&self, at: Vector2<usize>) -> usize {
        at.y * COLUMNS + at.x
    }

    pub fn row(&self, row: usize) -> &[T]{
        let row_start = row * COLUMNS;
        &self.data[row_start..row_start + COLUMNS]
    }

    pub fn get_pixel_mut(&mut self, at: Vector2<usize>) -> &mut char {
        &mut self.data[self.index(at)]
    }

    pub fn get_pixel(&self, at: Vector2<usize>) -> &char {
        &self.data[self.index(at)]
    }

    pub fn width(&self) -> usize {
        COLUMNS
    }

    pub fn height(&self) -> usize {
        ROWS
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}