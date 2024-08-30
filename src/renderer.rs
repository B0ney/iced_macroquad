mod layer;
pub mod state;
pub mod quad;

use iced_core::renderer::Quad;
use iced_core::{Background, Color, Font, Pixels, Point, Rectangle, Transformation};
use macroquad::miniquad::Context;
use state::State;


pub struct Renderer {
    state: State,
    layers: layer::Stack,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            state: State::new(ctx),
            layers: layer::Stack::new(),
        }
    }

    pub fn present(&mut self) {

    }
}

impl iced_core::Renderer for Renderer {
    fn start_layer(&mut self, bounds: Rectangle) {
        self.layers.push_clip(bounds);
    }

    fn end_layer(&mut self) {
        self.layers.pop_clip();
    }

    fn start_transformation(&mut self, transformation: Transformation) {
        self.layers.push_transformation(transformation);
    }

    fn end_transformation(&mut self) {
        self.layers.pop_transformation();
    }

    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>) {
        let (layer, transformation) = self.layers.current_mut();
        layer.draw_quad(quad, background.into(), transformation);
    }

    fn clear(&mut self) {
        self.layers.clear()
    }
}

impl iced_core::text::Renderer for Renderer {
    type Font = Font;
    type Paragraph = ();
    type Editor = ();

    const ICON_FONT: Self::Font = Font::DEFAULT;

    const CHECKMARK_ICON: char = '0';

    const ARROW_DOWN_ICON: char = '0';

    fn default_font(&self) -> Self::Font {
        Font::default()
    }

    fn default_size(&self) -> Pixels {
        Pixels(16.0)
    }

    fn fill_paragraph(
        &mut self,
        text: &Self::Paragraph,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    ) {
    }

    fn fill_editor(
        &mut self,
        editor: &Self::Editor,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    ) {
    }

    fn fill_text(
        &mut self,
        text: iced_core::Text<String, Self::Font>,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    ) {
    }
}
