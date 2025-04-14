use alloc::string::String;
use pd::graphics::text::draw_text;
use simple_vector2::Vector2;
use game_logic::renderer::Renderer;

const FONT_METRICS: Vector2<usize> = Vector2::new(8, 11);
const WIDTH: usize = 400 / FONT_METRICS.x;
const HEIGHT: usize = 240 / FONT_METRICS.y;
const BUFFER_SIZE: usize = WIDTH * HEIGHT;


pub struct AsciiRenderer {
    buffer: [char; BUFFER_SIZE],
    character_metrics: Vector2<usize>,
}

impl AsciiRenderer {
    fn new(fill: char, character_metrics: Vector2<usize>) -> Self {
        Self {
            buffer: [fill; BUFFER_SIZE],
            character_metrics,
        }
    }

    pub fn index(&self, at: Vector2<usize>) -> usize {
        at.y * WIDTH + at.x
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item=char>{
        let row_start = row * WIDTH;
        (row_start..row_start + WIDTH).map(|x| self.buffer[x])
    }
}

impl Renderer for AsciiRenderer {
    fn get_pixel_mut(&mut self, at: Vector2<usize>) -> &mut char {
        &mut self.buffer[self.index(at)]
    }

    fn get_pixel(&self, at: Vector2<usize>) -> &char {
        &self.buffer[self.index(at)]
    }

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    fn clear(&mut self, with: Option<char>) {
        let with = with.unwrap_or(' ');
        self.buffer = [with; BUFFER_SIZE];
    }

    fn render<RenderingContext>(&mut self, _rendering_context: &mut RenderingContext) {
        for row in 0..HEIGHT {
            let row_text:String = self.row(row).collect();
            let _ = draw_text(row_text, 0, (row * self.character_metrics.y) as i32);
        }
    }
}

impl Default for AsciiRenderer {
    fn default() -> Self {
        AsciiRenderer::new('.', Vector2::new(8, 11))
    }
}