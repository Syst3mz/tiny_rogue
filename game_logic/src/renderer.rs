use core::ops::Range;
use line_drawing::Bresenham;
use simple_vector2::Vector2;
use crate::conversions::Vector2ToTuple;

pub trait Renderer {
    fn get_pixel_mut(&mut self, at: Vector2<usize>) -> &mut char;
    fn get_pixel(&self, at: Vector2<usize>) -> &char;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn clear(&mut self, with: Option<char>);
    fn bounds(&self) -> Vector2<usize> {
        Vector2::new(self.width(), self.height())
    }
    fn last_valid_indices(&self) -> Vector2<usize> {
        Vector2::new(self.width() - 1, self.height() - 1)
    }
    fn set_pixel(&mut self, at: Vector2<usize>, pixel: char) {
        *self.get_pixel_mut(at) = pixel;
    }

    fn line(&mut self, from: Vector2<usize>, to: Vector2<usize>, with: char) {
        let line = Bresenham::new(
            from.map(|x| x as isize).to_tuple(),
            to.map(|x| x as isize).to_tuple()
        );

        for (x, y) in line {
            if x < 0 || y < 0 {
                return;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= self.width() || y >= self.height() {
                return;
            }

            self.set_pixel(Vector2::new(x, y), with.clone());
        }
    }
    fn border(&mut self, top_left: Vector2<usize>, bottom_right: Vector2<usize>, with: char) {
        let bottom_right = Vector2::new(
            bottom_right.x.min(self.width()), bottom_right.y.min(self.height())
        );
        let span = bottom_right - top_left;
        let (span_x, span_y) = (
            Vector2::new(span.x, 0),
            Vector2::new(0, span.y)
        );

        self.line(top_left, top_left + span_x, with.clone());
        self.line(top_left + span_x, bottom_right, with.clone());
        self.line(top_left + span_y, bottom_right, with.clone());
        self.line(top_left, top_left + span_y, with.clone());
    }

    fn print(&mut self, string: impl AsRef<str>, at: Vector2<usize>) {
        let mut x_position = at.x;
        for char in string.as_ref().chars() {
            if x_position >= self.width() {
                return;
            }

            self.set_pixel(Vector2::new(x_position, at.y), char);
            x_position += 1;
        }
    }
    fn copy_from_slice_offset(&mut self, offset: usize, source: &[char]);
    fn render<RenderingContext>(&mut self, rendering_context: &mut RenderingContext);
}