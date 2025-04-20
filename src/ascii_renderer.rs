use alloc::string::String;
use core::mem;
use core::ops::Range;
use pd::graphics::text::draw_text;
use simple_vector2::Vector2;
use shared::constants::{DRAWING_BUFFER_SIZE, SCREEN_SIZE};
use game_logic::renderer::Renderer;
use shared::array_2d::Array2d;

pub struct AsciiRenderer {
    buffer: Array2d<DRAWING_BUFFER_SIZE, char>,
    character_metrics: Vector2<usize>,
}

impl AsciiRenderer {
    fn new(fill: char, character_metrics: Vector2<usize>) -> Self {
        Self {
            buffer: Array2d::new([fill; DRAWING_BUFFER_SIZE], SCREEN_SIZE),
            character_metrics,
        }
    }

    pub fn index(&self, at: Vector2<usize>) -> usize {
        at.y * SCREEN_SIZE.x + at.x
    }
}

impl Renderer for AsciiRenderer {
    fn get_pixel_mut(&mut self, at: Vector2<usize>) -> &mut char {
        self.buffer.get_pixel_mut(at).unwrap()
    }

    fn get_pixel(&self, at: Vector2<usize>) -> &char {
        self.buffer.get_pixel(at).unwrap()
    }

    fn width(&self) -> usize {
        SCREEN_SIZE.x
    }

    fn height(&self) -> usize {
        SCREEN_SIZE.y
    }

    fn clear(&mut self, with: Option<char>) {
        let with = with.unwrap_or('.');
        self.buffer.clear(with)
    }

    fn copy_from_slice_offset(&mut self, offset: usize, source: &[char]) {
        self.buffer.copy_from_slice_offset(offset, source)
    }

    fn render<RenderingContext>(&mut self, _rendering_context: &mut RenderingContext) {
        for row in 0..SCREEN_SIZE.y {
            let row_text:String = self.buffer.row(row).into_iter().collect();
            let _ = draw_text(row_text, 0, (row * self.character_metrics.y) as i32);
        }
    }
}

impl Default for AsciiRenderer {
    fn default() -> Self {
        AsciiRenderer::new('.', Vector2::new(8, 11))
    }
}